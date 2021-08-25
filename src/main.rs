use tokio::sync::mpsc;
use anyhow::Result;
use async_compat::CompatExt;
use futures::Sink;
use rust_examples::logger::setup_logs;
use tracing::level_filters::LevelFilter;
use tracing::*;
use pin_utils::pin_mut;
use rust_examples::poll_data::fetch_from_polygon;
use rust_examples::analyze::{thread2, thread3};
use rust_examples::YourDataStruct;

const CHANNEL_BUFFER_SIZE: usize = 8; //capacity of the channels


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

