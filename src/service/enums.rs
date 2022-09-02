pub fn get_enums() -> Vec<Type> {
    vec![
        Type::Enum(
            "role".to_owned(),
            vec![
                EnumVariant::new("guest", 0),
                EnumVariant::new("user", 1),
                EnumVariant::new("admin", 2),
                EnumVariant::new("developer", 3),
            ],
        ),
        Type::Enum(
            "recovery_question_category".to_owned(),
            vec![
                EnumVariant::new("childhood", 0),
                EnumVariant::new("education", 1),
                EnumVariant::new("family", 2),
                EnumVariant::new("favorite", 3),
                EnumVariant::new("first", 4),
                EnumVariant::new("personal", 5),
                EnumVariant::new("pet", 6),
                EnumVariant::new("work", 7),
                EnumVariant::new("historical", 8),
            ],
        ),
    ]
}
