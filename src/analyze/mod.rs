use tokio::sync::mpsc;
use anyhow::Result;
use crate::YourDataStruct;
#[allow(unused_imports)]
use tracing::*;
//expensive computation, runs on the rayon thread pool
pub fn thread2(mut rx_t1: mpsc::Receiver<String>, tx_t2: mpsc::Sender<YourDataStruct>) -> Result<()> {
    while let Some(body) = rx_t1.blocking_recv() { //get the result from the previous thread.
        let data: ijson::IValue = serde_json::from_str(&body)?;
        info!("Parsed data {:?}", data);
        //do your first data processing here...
        tx_t2.blocking_send(YourDataStruct::default())?; //send the result to the next thread.
    }
    Ok(())
}

//expensive computation, runs on the rayon thread pool
pub fn thread3(mut rx_t2: mpsc::Receiver<YourDataStruct>, tx_t3: mpsc::Sender<YourDataStruct>) -> Result<()> {
    while let Some(data) = rx_t2.blocking_recv() { //get the result from the previous thread.
        //do your second data processing here...
        tx_t3.blocking_send(data)?; //send the result to the next thread.
    }
    Ok(())
}
