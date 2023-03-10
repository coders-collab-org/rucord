use std::{env, sync::Arc};

use async_trait::async_trait;
use rucord_api_types::GatewayIntentBits;
use rucord_rest::{RequestManager, RequestManagerOptions};
use rucord_ws::{
    Result, ShardError, WebSocketEventHandler, WebSocketManager, WebSocketManagerOptions,
};

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("BOT_TOKEN").expect("expected BOT_TOKEN env.");

    let rest = Arc::new(
        RequestManager::new_with_token(RequestManagerOptions::default(), token.clone()).into(),
    );

    let intents = GatewayIntentBits::MessageContent;

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
        eprintln!("[ERROR] [SHARD {id}] {error}");
    }
}
