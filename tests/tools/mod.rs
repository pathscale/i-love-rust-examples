use eyre::*;
use lib::ws::WsClient;
pub async fn get_ws_client() -> Result<WsClient> {
    let connect_addr = "ws://localhost:8888";

    let ws_stream = WsClient::new(connect_addr).await?;
    Ok(ws_stream)
}
