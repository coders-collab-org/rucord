use std::{sync::Arc, time::Instant};

use crate::{
    Result, Session, ShardError, ShardId, WebSocket, WebSocketError, WebSocketEventHandler,
    WebSocketExt, WebSocketWorkerOptions, WorkerMessage,
};
use async_tungstenite::tungstenite::protocol::CloseFrame;
use kanal::{AsyncReceiver, AsyncSender};
use rand::Rng;
use rucord_api_types::{GatewayReceivePayload, GatewaySendPayload, IdentifyData};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WebSocketShardStatus {
    Ready,
    Resuming,
    Connecting,
    Idle,
}

pub enum ShardMessage {
    Connected,
    Destroyed,
}

pub enum ShardSendMessage {
    Debug(ShardId, String),
    Error(ShardError),
}
pub struct WebSocketShard {
    id: ShardId,

    options: Arc<WebSocketWorkerOptions>,

    status: WebSocketShardStatus,

    session: Option<Session>,

    event_handler: Arc<dyn WebSocketEventHandler>,

    receiver: AsyncReceiver<WorkerMessage>,

    sender: AsyncSender<ShardMessage>,

    connection: Option<WebSocket>,

    started_at: Instant,

    last_heartbeat: Instant,

    heartbeat_interval: i64,

    next_heartbeat: i64,

    is_ack: bool,
}

impl WebSocketShard {
    pub fn new(
        id: ShardId,
        options: Arc<WebSocketWorkerOptions>,
        receiver: AsyncReceiver<WorkerMessage>,
        sender: AsyncSender<ShardMessage>,
    ) -> Self {
        Self {
            id,
            event_handler: options.event_handler.clone(),
            options,
            receiver,
            sender,
            status: WebSocketShardStatus::Idle,
            connection: None,
            started_at: Instant::now(),
            last_heartbeat: Instant::now(),
            heartbeat_interval: -1,
            next_heartbeat: -1,
            session: None,
            is_ack: true,
        }
    }
}

impl WebSocketShard {
    #[inline]
    pub fn status(&self) -> WebSocketShardStatus {
        self.status
    }

    #[inline]
    pub async fn debug(&self, msg: &[impl AsRef<str>]) {
        let msg = msg
            .iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<String>>()
            .join("\n");
        self.event_handler
            .debug(self.id, format!("[DEBUG] [SHARD {}]: {}", self.id, msg))
            .await;
    }

    #[inline]
    pub async fn error(&self, err: &WebSocketError) {
        if let WebSocketError::Shard(err) = err {
            self.event_handler.shard_error(self.id, err).await;
        } else {
            println!("{err}");
        }
    }
    pub async fn connect(&mut self) -> Result<()> {
        if self.status != WebSocketShardStatus::Idle {
            Err(ShardError::NotIdle)?;
        }

        let started_at = Instant::now();

        self.debug(&["Started WebSocket connection."]).await;

        let connection = WebSocket::create(&self.options.gateway_info.lock().await.url).await?;

        let take = started_at.elapsed().as_millis();

        self.debug(&[format!("WebSocket connection established after {take} ms.")])
            .await;

        self.started_at = started_at;

        self.connection = Some(connection);

        Ok(())
    }

    pub async fn destroy(&mut self, info: Option<CloseFrame<'static>>) {
        if self.status == WebSocketShardStatus::Idle {
            return;
        }

        let Some(ref mut connection) = self.connection else { return; };

        if connection.close(info).await.is_err() {}
    }

    pub async fn event_loop(&mut self) -> Result<()> {
        loop {
            // TODO: Resolve errors.
            match self.wait_worker_event().await {
                Ok(e) => match e {
                    WorkerMessage::Connect => {
                        let Err(err) = self.connect().await else {

                        if self.sender.send(ShardMessage::Connected).await.is_err() {
                            return Ok(());
                        };
                        continue;
                     };
                        self.resolve_ws_error(err).await?;
                    }

                    WorkerMessage::Destroy(info) => {
                        self.destroy(info).await;

                        if self.sender.send(ShardMessage::Destroyed).await.is_err() {
                            return Ok(());
                        };

                        return Ok(());
                    }
                },
                Err(e) if e == true => return Ok(()),
                _ => (),
            }

            if self.connection.is_some() && self.heartbeat_interval != -1 {
                if let Err(e) = self.heartbeat(false).await {
                    self.resolve_ws_error(e).await?;
                };
            }

            let Some(ref mut connection) = self.connection else { continue; };

            match connection.recv_next().await {
                Ok(Some(e)) => self.resolve_ws_event(e).await?,
                Ok(None) => continue,

                Err(err) => self.resolve_ws_error(err).await?,
            };
        }
    }

    pub async fn heartbeat(&mut self, requested: bool) -> Result<()> {
        if !requested && self.last_heartbeat.elapsed().as_millis() <= self.next_heartbeat as u128 {
            return Ok(());
        }

        println!("heartbeat: {}", self.next_heartbeat as u128);

        self.send(GatewaySendPayload::Heartbeat(
            self.session.as_ref().map(|s| s.sequence),
        ))
        .await?;

        self.last_heartbeat = Instant::now();

        self.next_heartbeat =
            (self.heartbeat_interval as f64 * rand::thread_rng().gen::<f64>()) as i64;

        self.is_ack = false;

        Ok(())
    }
    // TODO: Resolve ws event.
    pub async fn resolve_ws_event(&mut self, event: GatewayReceivePayload) -> Result<()> {
        match event {
            GatewayReceivePayload::Hello(heartbeat_interval) => {
                self.debug(&[format!(
                    "Initiating a regular heartbeat at an interval of {heartbeat_interval} ms."
                )])
                .await;

                self.heartbeat_interval = heartbeat_interval as i64;

                self.next_heartbeat =
                    (self.heartbeat_interval as f64 * rand::thread_rng().gen::<f64>()) as i64;

                self.last_heartbeat = Instant::now();

                self.identify().await?;
            }

            GatewayReceivePayload::HeartbeatRequest => self.heartbeat(true).await?,

            GatewayReceivePayload::HeartbeatAck => {
                self.is_ack = true;

                let latency = self.last_heartbeat.elapsed();

                self.debug(&[format!(
                    "The latency since the last heartbeat is: {}ms.",
                    latency.as_millis()
                )])
                .await;
            }

            a => {
                println!("event unimplemented yet {a:#?}");

                return Ok(());
            }
        }
        Ok(())
    }

    // TODO: Resolve error.
    pub async fn resolve_ws_error(&mut self, error: WebSocketError) -> Result<()> {
        self.error(&error).await;
        // match error {
        //     WebSocketError::Request(_) => todo!(),
        //     WebSocketError::Shard(_) => todo!(),
        //     WebSocketError::NotEnoughSessionsRemaining(_, _) => todo!(),
        //     WebSocketError::Json(_) => todo!(),
        // }

        Err(error)
    }
    pub async fn wait_worker_event(
        &mut self,
    ) -> core::result::Result<WorkerMessage, /*need_to_stop: */ bool> {
        if self.connection.is_some() {
            match self.receiver.try_recv() {
                Ok(Some(e)) => Ok(e),
                Ok(None) => Err(false),
                _ => Err(true),
            }
        } else {
            match self.receiver.recv().await {
                Ok(e) => Ok(e),
                _ => Err(true),
            }
        }
    }

    pub async fn identify(&mut self) -> Result<()> {
        let WebSocketWorkerOptions {
            token,
            identify_properties,
            intents,
            gateway_info,
            ..
        } = self.options.as_ref();

        self.debug(&[
            format!("Identifying"),
            format!("shard id: {}", self.id),
            format!("intents: {}", intents.bits()),
        ])
        .await;

        let data = IdentifyData {
            token: token.clone(),
            intents: intents.bits(),
            properties: identify_properties.clone(),

            shard: Some((self.id as u64, gateway_info.lock().await.shards)),

            ..Default::default()
        };

        self.send(GatewaySendPayload::Identify(data)).await
    }

    pub async fn send(&mut self, op: GatewaySendPayload) -> Result<()> {
        self.connection
            .as_mut()
            .expect("Expected WebSocket Connection")
            .send_op(op)
            .await?;

        Ok(())
    }
}
