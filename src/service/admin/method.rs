use crate::endpoints::*;
use eyre::Context;
use gen::database::*;
use lib::toolbox::*;
use lib::ws::*;
use std::sync::Arc;

pub struct ListUsersHandler;

impl RequestHandler for ListUsersHandler {
    type Request = ListUsersRequest;
    type Response = ListUsersResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _conn: Arc<Connection>,
        req: WsRequestGeneric<Self::Request>,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            let result = db
                .fun_admin_list_users(FunAdminListUsersReq {
                    offset: req.params.offset as _,
                    limit: req.params.limit as _,
                })
                .await?;

            Ok(ListUsersResponse {
                users: result
                    .rows
                    .into_iter()
                    .map(|x| ListUsersResponseRow {
                        user_public_id: x.user_public_id,
                        username: x.username,
                        email: x.email,
                        created_at: x.created_at as _,
                        updated_at: x.updated_at as _,
                    })
                    .collect(),
            })
        });
    }
}

pub struct AssignRoleHandler;

impl RequestHandler for AssignRoleHandler {
    type Request = AssignRoleRequest;
    type Response = AssignRoleResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _conn: Arc<Connection>,
        req: WsRequestGeneric<Self::Request>,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            let result = db
                .fun_admin_assign_role(FunAdminAssignRoleReq {
                    operator_user_id: _conn.get_user_id() as _,
                    new_role: req
                        .params
                        .new_role
                        .parse()
                        .context("Failed to parse role")?,
                    user_public_id: req.params.user_public_id,
                })
                .await?;
            drop(result);
            Ok(AssignRoleResponse { success: true })
        });
    }
}
