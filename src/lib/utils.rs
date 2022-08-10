use std::fmt::Debug;
use std::future::Future;
use tracing::*;

pub async fn error_handled<T, Err: Debug>(fut: impl Future<Output=Result<T, Err>>) {
    error_handled_sync(fut.await);
}
pub fn error_handled_sync<T, Err: Debug>(result: Result<T, Err>) {
    match result {
        Ok(_) => {},
        Err(err) => error!("Error happened {:?}", err)
    }
}