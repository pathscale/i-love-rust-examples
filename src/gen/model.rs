use num_derive::FromPrimitive;
use serde::*;
use strum_macros::EnumString;
use tokio_postgres::types::*;

#[derive(
    Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, EnumString,
)]
#[postgres(name = "enum_role")]
pub enum EnumRole {
    #[postgres(name = "guest")]
    Guest = 0,
    #[postgres(name = "user")]
    User = 1,
    #[postgres(name = "admin")]
    Admin = 2,
    #[postgres(name = "developer")]
    Developer = 3,
}
#[derive(
    Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, EnumString,
)]
#[postgres(name = "enum_recovery_question_category")]
pub enum EnumRecoveryQuestionCategory {
    #[postgres(name = "childhood")]
    Childhood = 0,
    #[postgres(name = "education")]
    Education = 1,
    #[postgres(name = "family")]
    Family = 2,
    #[postgres(name = "favorite")]
    Favorite = 3,
    #[postgres(name = "first")]
    First = 4,
    #[postgres(name = "personal")]
    Personal = 5,
    #[postgres(name = "pet")]
    Pet = 6,
    #[postgres(name = "work")]
    Work = 7,
    #[postgres(name = "historical")]
    Historical = 8,
}
#[derive(
    Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, EnumString,
)]
#[postgres(name = "enum_service")]
pub enum EnumService {
    #[postgres(name = "auth")]
    Auth = 1,
    #[postgres(name = "user")]
    User = 2,
    #[postgres(name = "admin")]
    Admin = 3,
}
