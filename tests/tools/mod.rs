use eyre::*;
use lib::ws::WsClient;
pub async fn get_ws_auth_client(header: &str) -> Result<WsClient> {
    let connect_addr = "ws://localhost:8888";
    println!("Connecting to {} with header {}", connect_addr, header);
    let ws_stream = WsClient::new(connect_addr, header).await?;
    Ok(ws_stream)
}

pub async fn get_ws_user_client(header: &str) -> Result<WsClient> {
    let connect_addr = "ws://localhost:8889";
    println!("Connecting to {} with header {}", connect_addr, header);
    let ws_stream = WsClient::new(connect_addr, header).await?;
    Ok(ws_stream)
}
pub async fn get_ws_admin_client(header: &str) -> Result<WsClient> {
    let connect_addr = "ws://localhost:8890";
    println!("Connecting to {} with header {}", connect_addr, header);
    let ws_stream = WsClient::new(connect_addr, header).await?;
    Ok(ws_stream)
}
