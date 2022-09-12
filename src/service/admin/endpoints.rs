use model::endpoint::*;
use model::types::{Field, Type};
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersRequest {
    pub offset: u32,
    pub limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersResponseRow {
    pub user_public_id: i64,
    pub username: String,
    pub email: String,
    pub created_at: u32,
    pub updated_at: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersResponse {
    pub users: Vec<ListUsersResponseRow>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssignRoleRequest {
    pub user_public_id: i64,
    pub new_role: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssignRoleResponse {
    pub success: bool,
}

pub fn endpoint_admin_list_users() -> EndpointSchema {
    EndpointSchema::new(
        "ListUsers",
        30010,
        vec![
            Field::new("offset", Type::Int),
            Field::new("limit", Type::Int),
        ],
        vec![
            Field::new("user_public_id", Type::BigInt),
            Field::new("username", Type::Text),
            Field::new("email", Type::Text),
            Field::new("created_at", Type::Int),
            Field::new("updated_at", Type::Int),
        ],
    )
}
pub fn endpoint_admin_assign_role() -> EndpointSchema {
    EndpointSchema::new(
        "AssignRole",
        30020,
        vec![
            Field::new("user_public_id", Type::BigInt),
            Field::new("new_role", Type::Text),
        ],
        vec![Field::new("success", Type::Boolean)],
    )
}
pub fn get_admin_endpoints() -> Vec<EndpointSchema> {
    vec![endpoint_admin_list_users(), endpoint_admin_assign_role()]
}
