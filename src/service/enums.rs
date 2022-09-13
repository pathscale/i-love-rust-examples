use crate::services::get_services;
use model::types::*;

pub fn get_service_enum() -> Type {
    Type::enum_(
        "service".to_owned(),
        get_services()
            .iter()
            .map(|s| EnumVariant::new(s.name.clone(), s.id as _))
            .collect::<Vec<EnumVariant>>(),
    )
}
pub fn get_enums() -> Vec<Type> {
    vec![
        Type::enum_(
            "role".to_owned(),
            vec![
                EnumVariant::new("guest", 0),
                EnumVariant::new("user", 1),
                EnumVariant::new("admin", 2),
                EnumVariant::new("developer", 3),
            ],
        ),
        Type::enum_(
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
        get_service_enum(),
    ]
}
