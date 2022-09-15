use gen::model::*;
use lib::toolbox::*;
use lib::ws::*;
use std::sync::Arc;

pub struct FooHandler;

impl RequestHandler for FooHandler {
    type Request = FooRequest;
    type Response = FooResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _conn: Arc<Connection>,
        _req: Self::Request,
    ) {
        toolbox.spawn_response(ctx, async move { Ok(FooResponse { foo: false }) });
    }
}
