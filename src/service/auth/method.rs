use eyre::*;
use futures::future::BoxFuture;
use futures::FutureExt;
use lib::ws::*;
use serde::*;
use std::sync::Arc;

pub struct HandleAuthFoo;
#[derive(Serialize)]
pub struct AuthFooResp {
    user: String,
}
impl AsyncRequestHandler for HandleAuthFoo {
    type Request = ();
    type Response = AuthFooResp;

    fn handle(
        &self,
        _conn: Arc<Connection>,
        _req: WsRequestGeneric<Self::Request>,
    ) -> BoxFuture<'static, Result<Self::Response>> {
        async move {
            Ok(AuthFooResp {
                user: "jack".to_string(),
            })
        }
        .boxed()
    }
}
