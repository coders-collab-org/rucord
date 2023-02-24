use async_trait::async_trait;
use async_tungstenite::{
    tokio::{connect_async, ConnectStream},
    tungstenite::Message,
    WebSocketStream,
};
use futures::StreamExt;
use rucord_api_types::GatewayReceivePayload;
use tokio::time::{timeout, Duration};

use crate::{Result, ShardError, ShardId};

pub type WebSocket = WebSocketStream<ConnectStream>;

#[async_trait]
pub trait WebSocketExt {
    async fn create<T: AsRef<str> + Send + Sync>(url: T) -> WebSocket {
        let (ws_stream, _) = connect_async(url.as_ref()).await.unwrap();
        ws_stream
    }
    async fn recv_next(&mut self) -> Result<Option<GatewayReceivePayload>>;
}

#[async_trait]
impl WebSocketExt for WebSocket {
    async fn recv_next(&mut self) -> Result<Option<GatewayReceivePayload>> {
        const TIME: Duration = Duration::from_millis(500);

        match timeout(TIME, self.next()).await {
            Ok(Some(Ok(v))) => Ok(get_text(v)?.map(GatewayReceivePayload::unpack)),
            Ok(Some(Err(e))) => Err(ShardError::Tungstenite(e))?,
            Ok(None) | Err(_) => Ok(None),
        }
    }
}

fn get_text(msg: Message) -> Result<Option<String>> {
    match msg {
        Message::Text(txt) => Ok(Some(txt)),

        // TODO: Compress data.
        Message::Binary(_) => unimplemented!(),

        Message::Close(frame) => Err(ShardError::Closed(frame))?,

        _ => Ok(None),
    }
}

// TODO: Use debug method in the code.
#[async_trait]
pub trait WebSocketEventHandler: Send + Sync {
    async fn debug(&self, _shard_id: ShardId, _message: String) {}
    async fn shard_error(&self, _shard_id: ShardId, _error: ShardError) {}
}
