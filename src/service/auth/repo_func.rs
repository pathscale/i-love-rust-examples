use model::types::*;

pub fn get_auth_repo_func() -> Vec<RepositoryFunction> {
    vec![
        RepositoryFunction::new(
            "fun_auth_signup",
            vec![
                Field::new("public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("email", Type::String),
                Field::new("phone", Type::String),
                Field::new("password_hash", Type::Bytea),
                Field::new("password_salt", Type::Bytea),
                Field::new("age", Type::Int),
                Field::new("preferred_language", Type::String),
                Field::new("agreed_tos", Type::Boolean),
                Field::new("agreed_privacy", Type::Boolean),
                Field::new("ip_address", Type::Inet),
            ],
            vec![Field::new("user_id", Type::BigInt)],
        ),
        RepositoryFunction::new(
            "fun_auth_authenticate",
            vec![
                Field::new("username", Type::String),
                Field::new("password_hash", Type::Bytea),
                Field::new("service_code", Type::Int),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
            ],
        ),
        RepositoryFunction::new(
            "fun_auth_get_password_salt",
            vec![Field::new("username", Type::String)],
            vec![Field::new("salt", Type::Bytea)],
        ),
        RepositoryFunction::new(
            "fun_auth_set_token",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_token", Type::UUID),
                Field::new("admin_token", Type::UUID),
                Field::new("service_code", Type::Int),
            ],
            vec![],
        ),
        RepositoryFunction::new(
            "fun_auth_authorize",
            vec![
                Field::new("username", Type::String),
                Field::new("token", Type::UUID),
                Field::new("service", Type::enum_ref("service")),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("role", Type::enum_ref("role")),
            ],
        ),
        RepositoryFunction::new(
            "fun_auth_change_password",
            vec![
                Field::new("username", Type::String),
                Field::new("old_password_hash", Type::Bytea),
                Field::new("new_password_hash", Type::Bytea),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![],
        ),
        RepositoryFunction::new(
            "fun_get_recovery_question_data",
            vec![],
            vec![
                Field::new("question_id", Type::BigInt),
                Field::new("content", Type::String),
                Field::new("category", Type::enum_ref("recovery_question_category")),
            ],
        ),
        RepositoryFunction::new(
            "fun_auth_set_recovery_questions",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("question_ids", Type::Vec(Box::new(Type::BigInt))),
                Field::new("answers", Type::Vec(Box::new(Type::Int))),
            ],
            vec![],
        ),
        RepositoryFunction::new(
            "fun_auth_basic_authenticate",
            vec![
                Field::new("username", Type::String),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![Field::new("user_id", Type::Inet)],
        ),
        RepositoryFunction::new(
            "fun_auth_get_recovery_questions",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("question_id", Type::BigInt),
                Field::new("question", Type::String),
            ],
        ),
        RepositoryFunction::new(
            "fun_submit_recovery_answers",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("question_ids", Type::Vec(Box::new(Type::BigInt))),
                Field::new("answers", Type::Vec(Box::new(Type::String))),
                Field::new("password_reset_token", Type::UUID),
                Field::new("token_valid", Type::Int),
            ],
            vec![],
        ),
        RepositoryFunction::new(
            "fun_auth_reset_password",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("new_password_hash", Type::Bytea),
                Field::new("new_password_salt", Type::Bytea),
                Field::new("reset_token", Type::UUID),
            ],
            vec![],
        ),
    ]
}
