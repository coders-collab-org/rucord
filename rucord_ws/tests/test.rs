use std::sync::Arc;

use async_trait::async_trait;
use rucord_api_types::GatewayIntentBits;
use rucord_rest::{RequestManager, RequestManagerOptions};
use rucord_ws::{ShardError, WebSocketEventHandler, WebSocketManager, WebSocketManagerOptions};

pub fn get_token() -> String {
    include_str!("./token.private").into()
}

#[actix_rt::test]
pub async fn test() {
    let token = get_token();

    let rest = Arc::new(
        RequestManager::new_with_token(RequestManagerOptions::default(), token.clone()).into(),
    );

    let intents = GatewayIntentBits::MessageContent;

    let mut ws = WebSocketManager::new(WebSocketManagerOptions {
        token,
        intents,
        rest,
    });

    let Err(err) = ws.connect(RawEventHandler).await else {
        return;
    };

    eprintln!("{err}")
}

struct RawEventHandler;

#[async_trait]
impl WebSocketEventHandler for RawEventHandler {
    async fn debug(&self, _: usize, message: String) {
        println!("{message}")
    }
    async fn shard_error(&self, id: usize, error: ShardError) {
        eprintln!("[ERROR] [Shard {id}]: {error}");
    }
}