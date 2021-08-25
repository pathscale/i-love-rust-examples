use std::{
    time::Duration,
    io,
};
use tokio::sync::mpsc::{self, error::SendError};
use paperplane::Server;
use reqwest::Client;
use nonzero_ext::nonzero;
use governor::{RateLimiter, Quota};

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
    while let Some(body) = rx_t1.blocking_recv() { //get the result from the previous thread.
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

async fn websocket_send(mut rx_t3: mpsc::Receiver<YourDataStruct>) -> io::Result<()> {
    let server = Server::new(10); //change the value as you like
    server.listen("0.0.0.0:4444").await?;
    while let Some(result) = rx_t3.recv().await { //get the result from the previous thread.
        server.send(None, format!("Result: {:?}", result)).await.unwrap();
    }
    Ok(())
}

#[tokio::main]
async fn main() {
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

    websocket_send(rx_t3).await.unwrap();
}

