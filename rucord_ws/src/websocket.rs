use std::time::Duration;

use async_trait::async_trait;
use async_tungstenite::{
    tokio::{connect_async, ConnectStream},
    tungstenite::Message,
    WebSocketStream,
};
use futures::{SinkExt, StreamExt};
use rucord_api_types::{GatewayReceivePayload, GatewaySendPayload};
use serde_json::to_string;
use tokio::time::timeout;

use crate::{Result, ShardError, ShardId};

pub type WebSocket = WebSocketStream<ConnectStream>;

#[async_trait]
pub trait WebSocketExt {
    async fn create<T: AsRef<str> + Send + Sync>(url: T) -> Result<WebSocket> {
        let (ws, _) = connect_async(url.as_ref())
            .await
            .map_err(ShardError::Tungstenite)?;

        Ok(ws)
    }
    async fn recv_next(&mut self) -> Result<Option<GatewayReceivePayload>>;
    async fn send_op(&mut self, op: GatewaySendPayload) -> Result<()>;
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

    async fn send_op(&mut self, op: GatewaySendPayload) -> Result<()> {
        self.send(Message::Text(to_string(&op)?))
            .await
            .map_err(ShardError::Tungstenite)?;
        Ok(())
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
    async fn shard_error(&self, _shard_id: ShardId, _error: &ShardError) {}
}
