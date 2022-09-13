use model::endpoint::*;
use model::types::{Field, Type};

pub fn endpoint_admin_list_users() -> EndpointSchema {
    EndpointSchema::new(
        "ListUsers",
        30010,
        vec![
            Field::new("offset", Type::Int),
            Field::new("limit", Type::Int),
        ],
        vec![Field::new(
            "users",
            Type::data_table(
                "ListUsersResponseRow".to_owned(),
                vec![
                    Field::new("user_public_id", Type::BigInt),
                    Field::new("username", Type::String),
                    Field::new("email", Type::String),
                    Field::new("created_at", Type::Int),
                    Field::new("updated_at", Type::Int),
                ],
            ),
        )],
    )
}
pub fn endpoint_admin_assign_role() -> EndpointSchema {
    EndpointSchema::new(
        "AssignRole",
        30020,
        vec![
            Field::new("user_public_id", Type::BigInt),
            Field::new("new_role", Type::String),
        ],
        vec![Field::new("success", Type::Boolean)],
    )
}
pub fn get_admin_endpoints() -> Vec<EndpointSchema> {
    vec![endpoint_admin_list_users(), endpoint_admin_assign_role()]
}
