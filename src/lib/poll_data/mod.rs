use tokio::sync::mpsc;
use reqwest::Client;
use nonzero_ext::nonzero;
use governor::{RateLimiter, Quota};
use eyre::Result;
#[allow(unused_imports)]
use tracing::*;
use std::time::Duration;

// This fetch JSON from Polygon
pub async fn fetch_from_polygon(tx_t1: mpsc::Sender<String>) -> Result<()>{
    let client = Client::builder()
        .tcp_keepalive(Some(Duration::from_secs(60))) //change the value as you like
        .build()?;
    let lim = RateLimiter::direct(
        Quota::per_minute(nonzero!(20u32))
            .allow_burst(nonzero!(1u32))
    );
    loop {
        lim.until_ready().await;
        let body = client.get("https://jsonplaceholder.typicode.com/users")
            .send()
            .await?
            .text()
            .await?;
        tx_t1.send(body).await.unwrap(); //sending the result to the next thread.
    }
}
