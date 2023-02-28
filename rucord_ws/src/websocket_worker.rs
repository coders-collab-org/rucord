use std::sync::Arc;

use async_tungstenite::tungstenite::protocol::CloseFrame;
use kanal::{AsyncReceiver, AsyncSender};
use rucord_api_types::{GatewayBotObject, GatewayIntentBits, IdentifyConnectionProperties};
use tokio::{spawn, sync::Mutex};

use crate::{IdentifyQueue, ShardId, ShardMessage, WebSocketEventHandler, WebSocketShard};

pub struct WebSocketWorkerOptions {
    pub gateway_info: Arc<Mutex<GatewayBotObject>>,

    pub token: String,

    pub identify_properties: IdentifyConnectionProperties,

    pub identify_queue: IdentifyQueue,

    pub event_handler: Arc<dyn WebSocketEventHandler>,

    pub intents: GatewayIntentBits,
}

pub enum WorkerMessage {
    Connect,
    Destroy(Option<CloseFrame<'static>>),
}

pub struct WebSocketWorker {
    pub id: ShardId,
    pub options: Arc<WebSocketWorkerOptions>,
    pub shard_sender: AsyncSender<WorkerMessage>,
    pub worker_receiver: AsyncReceiver<ShardMessage>,
}

impl WebSocketWorker {
    pub async fn new(id: ShardId, options: Arc<WebSocketWorkerOptions>) -> Self {
        let (shard_sender, shard_receiver) = kanal::unbounded_async();
        let (worker_sender, worker_receiver) = kanal::unbounded_async();

        let mut shard = WebSocketShard::new(id, options.clone(), shard_receiver, worker_sender);

        spawn(async move { shard.event_loop().await });

        Self {
            id,
            options,
            shard_sender,
            worker_receiver,
        }
    }

    pub async fn connect(&self) {
        if self
            .shard_sender
            .send(WorkerMessage::Connect)
            .await
            .is_err()
        {
            return;
        }

        loop {
            let Ok(msg) = self.worker_receiver.recv().await else { return; };
            if let ShardMessage::Connected = msg {
                return;
            }
        }
    }
    pub async fn destroy(&self, info: Option<CloseFrame<'static>>) {
        if self
            .shard_sender
            .send(WorkerMessage::Destroy(info))
            .await
            .is_err()
        {
            return;
        }

        loop {
            let Ok(msg) = self.worker_receiver.recv().await else { return; };
            if let ShardMessage::Destroyed = msg {
                return;
            }
        }
    }
}
