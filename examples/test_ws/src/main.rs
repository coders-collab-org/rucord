use std::{env, sync::Arc};

use async_trait::async_trait;
use rucord_api_types::GatewayIntentBits;
use rucord_rest::RequestManager;
use rucord_ws::{
    api_types, Result, ShardError, WebSocketEventHandler, WebSocketManager, WebSocketManagerOptions,
};

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("BOT_TOKEN").expect("expected BOT_TOKEN env.");

    let rest = Arc::new(RequestManager::new_with_token(Default::default(), token.clone()).into());

    let intents = GatewayIntentBits::MessageContent | GatewayIntentBits::Guilds;

    let mut ws = WebSocketManager::new(WebSocketManagerOptions {
        token,
        intents,
        rest,
    });

    ws.connect(RawEventHandler).await?;

    Ok(())
}

struct RawEventHandler;

#[async_trait]
impl WebSocketEventHandler for RawEventHandler {
    async fn debug(&self, _: usize, message: String) {
        println!("{message}")
    }
    async fn shard_error(&self, id: usize, error: &ShardError) {
        eprintln!("[ERROR] [SHARD {id}]: {error}");
    }

    async fn dispatch(&self, id: usize, data: &api_types::DispatchPayload) {
        println!("[INFO] [SHARD {id}]: new event {data:#?}")
    }
    async fn ready(&self, id: usize, data: &api_types::ReadyData) {
        println!("[INFO] [SHARD {id}]: shard is ready {data:#?}")
    }
    async fn resumed(&self, _id: usize) {
        // NO OP
    }
}
