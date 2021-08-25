use tokio::sync::mpsc::{self, error::SendError};
use reqwest::Client;
use nonzero_ext::nonzero;
use governor::{RateLimiter, Quota};
use anyhow::Result;
use async_compat::CompatExt;
use futures::Sink;
use rust_examples::logger::setup_logs;
use tracing::level_filters::LevelFilter;
use tracing::*;
use std::time::Duration;
use pin_utils::pin_mut;
const CHANNEL_BUFFER_SIZE: usize = 8; //capacity of the channels

#[derive(Default, Debug)]
struct YourDataStruct {
    //your awesome data...
}

// This fetch JSON from Polygon
async fn fetch_from_polygon(tx_t1: mpsc::Sender<String>) -> Result<(), reqwest::Error> {
    let client = Client::builder()
        .tcp_keepalive(Some(Duration::from_secs(60))) //change the value as you like
        .build()?;
    let lim = RateLimiter::direct(
        Quota::per_minute(nonzero!(20u32))
            .allow_burst(nonzero!(1u32))
    );
    loop {
        lim.until_ready().await;
        let body = client.get("https://www.rust-lang.org")
            .send()
            .await?
            .json()
            .await?;
        tx_t1.send(body).await.unwrap(); //sending the result to the next thread.
    }
}

//expensive computation, runs on the rayon thread pool
fn thread2(mut rx_t1: mpsc::Receiver<String>, tx_t2: mpsc::Sender<YourDataStruct>) -> Result<(), SendError<YourDataStruct>> {
    while let Some(_body) = rx_t1.blocking_recv() { //get the result from the previous thread.
        //do your first data processing here...
        tx_t2.blocking_send(YourDataStruct::default())?; //send the result to the next thread.
    }
    Ok(())
}

//expensive computation, runs on the rayon thread pool
fn thread3(mut rx_t2: mpsc::Receiver<YourDataStruct>, tx_t3: mpsc::Sender<YourDataStruct>) -> Result<(), SendError<YourDataStruct>> {
    while let Some(data) = rx_t2.blocking_recv() { //get the result from the previous thread.
        //do your second data processing here...
        tx_t3.blocking_send(data)?; //send the result to the next thread.
    }
    Ok(())
}

async fn websocket_send(_rx_t3: mpsc::Receiver<YourDataStruct>) -> Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444").await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted stream from {}", addr);
        tokio::spawn(async move {
            let result = async {
                let stream = async_tungstenite::accept_async(stream.compat()).await?;
                pin_mut!(stream);
                stream.as_mut().start_send(async_tungstenite::tungstenite::Message::Text("hello world".to_owned()))?;
                Ok::<(), anyhow::Error>(())
            }.await;
            if let Err(err) = result {
                error!(?addr, "Error while processing {}", err)
            }
        });
    }

}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logs(LevelFilter::INFO)?;
    info!("Starting server");
    let (tx_t1, rx_t1) = mpsc::channel(CHANNEL_BUFFER_SIZE);
    tokio::spawn(fetch_from_polygon(tx_t1));

    let (tx_t2, rx_t2) = mpsc::channel(CHANNEL_BUFFER_SIZE);

    rayon::spawn(|| {
        thread2(rx_t1, tx_t2).unwrap();
    });
    let (tx_t3, rx_t3) = mpsc::channel(CHANNEL_BUFFER_SIZE);
    rayon::spawn(|| {
        thread3(rx_t2, tx_t3).unwrap();
    });
    //repeat this steps for each threads you want to have...

    websocket_send(rx_t3).await?;
    Ok(())
}

