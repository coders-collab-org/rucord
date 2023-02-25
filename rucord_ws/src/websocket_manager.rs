use std::{sync::Arc, time::Instant};

use async_tungstenite::tungstenite::protocol::CloseFrame;
use futures::future::join_all;
use rucord_api_types::{GatewayBotObject, GatewayIntentBits, SessionStartLimitObject};
use rucord_rest::RequestManager;
use tokio::sync::Mutex;

use crate::{Result, ShardBucket, WebSocketError, WebSocketEventHandler, WebSocketWorkerOptions};

pub type ShardId = usize;

pub struct Session {
    pub id: String,

    pub shard_id: ShardId,

    pub resume_url: String,

    pub shard_count: u64,

    pub sequence: u64,
}

pub struct WebSocketManagerOptions {
    pub token: String,

    pub intents: GatewayIntentBits,

    pub rest: Arc<Mutex<RequestManager>>,
}

#[derive(Clone)]
struct GatewayInfo {
    pub info: Arc<Mutex<GatewayBotObject>>,
    pub created_at: Instant,
}

pub struct WebSocketManager {
    options: WebSocketManagerOptions,

    gateway_info: Option<GatewayInfo>,

    shard_ids: Option<Vec<ShardId>>,

    buckets: Vec<ShardBucket>,
}

impl WebSocketManager {
    pub fn new(options: WebSocketManagerOptions) -> Self {
        Self {
            options,
            gateway_info: None,
            shard_ids: None,
            buckets: vec![],
        }
    }
}

impl WebSocketManager {
    pub async fn fetch_gateway_info(&mut self) -> Result<Arc<Mutex<GatewayBotObject>>> {
        match self.gateway_info {
            Some(GatewayInfo {
                ref info,
                created_at,
            }) if (created_at.elapsed().as_millis() as u64)
                < info.lock().await.session_start_limit.reset_after =>
            {
                return Ok(info.clone());
            }
            _ => (),
        }

        let info = self.options.rest.lock().await.get_gateway_bot().await?;

        if let Some(ref mut gateway_info) = self.gateway_info {
            *gateway_info.info.lock().await = info;
        } else {
            self.gateway_info = Some(info.into());
        }

        Ok(self.gateway_info.as_ref().unwrap().info.clone())
    }

    pub async fn shard_ids(&mut self) -> Result<&Vec<usize>> {
        if let Some(ref shard_ids) = self.shard_ids {
            return Ok(shard_ids);
        }

        let data = self.fetch_gateway_info().await?;

        let shard_ids = (0..data.lock().await.shards).map(|i| i as usize).collect();

        self.shard_ids = Some(shard_ids);

        Ok(self.shard_ids.as_ref().unwrap())
    }

    pub async fn connect<T: WebSocketEventHandler + 'static>(
        &mut self,
        event_handler: T,
    ) -> Result<()> {
        let GatewayBotObject {
            shards,
            session_start_limit: SessionStartLimitObject { remaining, .. },
            ..
        } = self.fetch_gateway_info().await?.lock().await.clone();

        if shards > remaining {
            Err(WebSocketError::NotEnoughSessionsRemaining(
                remaining, shards,
            ))?;
        };

        self.shard_ids().await?;
        self.spawn(event_handler).await?;

        for bucket in self.buckets.iter() {
            bucket.connect().await;
        }

        loop {}
    }

    pub async fn destroy(&self, info: Option<CloseFrame<'static>>) {
        join_all(self.buckets.iter().map(|b| b.destroy(&info))).await;
    }

    async fn spawn<T: WebSocketEventHandler + 'static>(&mut self, event_handler: T) -> Result<()> {
        let event_handler = Arc::new(event_handler);

        let WebSocketManagerOptions { token, intents, .. } = &self.options;

        let gateway_info = self.gateway_info.as_ref().unwrap().info.clone();

        let bucket_size = gateway_info
            .lock()
            .await
            .session_start_limit
            .max_concurrency;

        let options = Arc::new(WebSocketWorkerOptions {
            gateway_info,
            event_handler,
            token: token.clone(),
            identify_properties: Default::default(),
            intents: intents.clone(),
        });
        self.buckets = join_all(
            self.shard_ids
                .as_ref()
                .unwrap()
                .chunks(bucket_size as usize)
                .map(|ids| ShardBucket::new(ids, options.clone())),
        )
        .await;

        Ok(())
    }
}

impl From<GatewayBotObject> for GatewayInfo {
    #[inline]
    fn from(info: GatewayBotObject) -> Self {
        Self {
            info: Arc::new(info.into()),
            created_at: Instant::now(),
        }
    }
}
