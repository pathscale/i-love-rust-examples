use gen::model::*;
use lib::database::LocalDbClient;
use lib::handler::RequestHandler;
use lib::toolbox::*;
use lib::ws::*;
use std::sync::Arc;

use super::repository;

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
        let db: LocalDbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            repository::fun_user_foo(&db, ()).await?;
            Ok(FooResponse { foo: false })
        });
    }
}
