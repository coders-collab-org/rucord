use std::{sync::Arc, time::Instant};

use crate::{
    Result, Session, ShardError, ShardId, WebSocket, WebSocketError, WebSocketEventHandler,
    WebSocketExt, WebSocketWorkerOptions, WorkerMessage,
};
use async_tungstenite::tungstenite::protocol::CloseFrame;
use kanal::{AsyncReceiver, AsyncSender};
use rucord_api_types::GatewayReceivePayload;

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
            session: None,
        }
    }
}

impl WebSocketShard {
    #[inline]
    pub fn status(&self) -> WebSocketShardStatus {
        self.status
    }

    pub async fn connect(&mut self) -> Result<()> {
        if self.status != WebSocketShardStatus::Idle {
            Err(ShardError::NotIdle)?;
        }
        let started_at = Instant::now();
        let connection = WebSocket::create(&self.options.gateway_info.lock().await.url).await;
        let take = started_at.elapsed().as_millis();

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
                        self.resolve_ws_error(err).await;
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

            let Some(ref mut connection) = self.connection else { continue; };

            match connection.recv_next().await {
                Ok(Some(e)) => self.resolve_ws_event(e)?,
                Ok(None) => continue,

                Err(err) => return Err(self.resolve_ws_error(err).await),
            };
        }
    }

    // TODO: Resolve ws event.
    pub fn resolve_ws_event(&mut self, event: GatewayReceivePayload) -> Result<()> {
        Ok(())
    }

    // TODO: Resolve error.
    pub async fn resolve_ws_error(&mut self, error: WebSocketError) -> WebSocketError {
        error
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
}
