use model::types::*;

pub fn get_admin_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_admin_list_users",
            vec![
                Field::new("offset", Type::Int),
                Field::new("limit", Type::Int),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("email", Type::Text),
                Field::new("username", Type::Text),
                Field::new("role", Type::Enum("role".to_string(), vec![])),
                Field::new("updated_at", Type::Int),
                Field::new("created_at", Type::Int),
            ],
            r#"
BEGIN
    RETURN QUERY SELECT
        u.user_id,
        u.user_public_id,
        u.email,
        u.username,
        u.role,
        u.updated_at::int,
        u.created_at::int
    FROM tbl.user AS u
    ORDER BY user_id
    OFFSET a_offset
    LIMIT a_limit;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_admin_assign_role",
            vec![
                Field::new("operator_user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
                Field::new("new_role", Type::Enum("role".to_owned(), vec![])),
            ],
            vec![],
            r#"
DECLARE
    operator_user enum_role; 
BEGIN
    SELECT role FROM tbl.user WHERE user_id = a_operator_user_id INTO STRICT operator_role;
    IF operator_role <> 'admin' THEN
        RAISE SQLSTATE R000S; -- InvalidRole
    END IF;
    UPDATE tbl.user SET role = a_new_role WHERE user_public_id = a_user_public_id;
END
        "#,
        ),
    ]
}
