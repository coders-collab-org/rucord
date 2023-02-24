use std::{collections::HashMap, sync::Arc};

use async_tungstenite::tungstenite::protocol::CloseFrame;
use futures::future::join_all;

use crate::{ShardId, WebSocketWorker, WebSocketWorkerOptions};

pub struct ShardBucket {
    pub workers: HashMap<ShardId, WebSocketWorker>,
}

impl ShardBucket {
    pub async fn new(ids: &[ShardId], worker_options: Arc<WebSocketWorkerOptions>) -> Self {
        let workers = join_all(
            ids.iter()
                .map(|id| WebSocketWorker::new(*id, worker_options.clone())),
        )
        .await;

        Self {
            workers: workers.into_iter().enumerate().collect(),
        }
    }

    #[inline]
    pub async fn connect(&self) {
        join_all(self.workers.values().map(|w| w.connect())).await;
    }
    #[inline]
    pub async fn destroy(&self, info: &Option<CloseFrame<'static>>) {
        join_all(self.workers.values().map(|w| w.destroy(info.clone()))).await;
    }
}
