use eyre::Context;
use gen::database::*;
use gen::model::*;
use lib::database::LocalDbClient;
use lib::handler::RequestHandler;
use lib::toolbox::*;
use lib::ws::*;
use std::sync::Arc;

pub struct ListUsersHandler;

use super::repository;

impl RequestHandler for ListUsersHandler {
    type Request = ListUsersRequest;
    type Response = ListUsersResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: LocalDbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            let rows = repository::fun_admin_list_users(
                &db,
                FunAdminListUsersReq {
                    offset: req.offset as _,
                    limit: req.limit as _,
                },
            )
            .await?
            .rows;

            Ok(ListUsersResponse {
                users: rows
                    .into_iter()
                    .map(|x| ListUsersResponseRow {
                        user_public_id: x.user_public_id,
                        email: x.email,
                        username: x.username,
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
        req: Self::Request,
    ) {
        let db: LocalDbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            repository::fun_admin_assign_role(
                &db,
                FunAdminAssignRoleReq {
                    operator_user_id: _conn.get_user_id() as _,
                    new_role: req.new_role.parse().context("Failed to parse role")?,
                    user_public_id: req.user_public_id,
                },
            )
            .await?;

            Ok(AssignRoleResponse { success: true })
        });
    }
}
