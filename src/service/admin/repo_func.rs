use model::types::*;

pub fn get_admin_repo_func() -> Vec<RepositoryFunction> {
    vec![
        RepositoryFunction::new(
            "fun_admin_list_users",
            vec![
                Field::new("offset", Type::Int),
                Field::new("limit", Type::Int),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("email", Type::String),
                Field::new("username", Type::String),
                Field::new("role", Type::enum_ref("role")),
                Field::new("updated_at", Type::BigInt),
                Field::new("created_at", Type::BigInt),
            ],
        ),
        RepositoryFunction::new(
            "fun_admin_assign_role",
            vec![
                Field::new("operator_user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("new_role", Type::enum_ref("role")),
            ],
            vec![],
        ),
    ]
}
