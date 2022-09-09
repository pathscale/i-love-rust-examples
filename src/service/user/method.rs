use crate::endpoints::*;
use gen::database::*;
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
        conn: Arc<Connection>,
        _req: WsRequestGeneric<Self::Request>,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {

            Ok(FooResponse {
                foo: false
            })
        });
    }
}
