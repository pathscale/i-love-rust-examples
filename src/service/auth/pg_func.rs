pub fn get_auth_pg_func() -> Vec<ProceduralFunction> {
    vec![ProceduralFunction::new(
        "fun_auth_add",
        vec![Field::new("a", Type::Int), Field::new("b", Type::Int)],
        vec![Field::new("sum", Type::Int)],
        r#"
BEGIN
    RETURN QUERY SELECT $a + $b;
END
        "#,
    )]
}
