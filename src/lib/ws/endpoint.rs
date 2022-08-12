use crate::model::WsEndpointSchema;
use crate::ws::basics::RequestHandlerRaw;
use std::sync::Arc;

pub struct WsEndpoint {
    pub schema: WsEndpointSchema,
    pub handler: Arc<dyn RequestHandlerRaw>,
}
