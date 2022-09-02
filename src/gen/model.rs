use serde::*;
use tokio_postgres::types::*;

#[derive(Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize)]
pub enum EnumRole {
    Guest = 0,
    User = 1,
    Admin = 2,
    Developer = 3,
}
#[derive(Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize)]
pub enum EnumRecoveryQuestionCategory {
    Childhood = 0,
    Education = 1,
    Family = 2,
    Favorite = 3,
    First = 4,
    Personal = 5,
    Pet = 6,
    Work = 7,
    Historical = 8,
}
#[derive(Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize)]
pub enum EnumService {
    Auth = 1,
    User = 2,
    Admin = 3,
}
