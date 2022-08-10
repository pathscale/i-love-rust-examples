use std::sync::Arc;
use crate::ws::basics::RequestHandler;

pub struct WsEndpoint {
    pub method: u32,
    pub handler: Arc<dyn RequestHandler>
}