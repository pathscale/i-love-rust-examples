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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub service_code: EnumService,
    pub device_id: String,
    pub device_os: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub username: String,
    pub user_public_id: i64,
    pub user_token: String,
    pub admin_token: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub agreed_tos: bool,
    pub agreed_privacy: bool,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignupResponse {
    pub username: String,
    pub user_public_id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    pub username: String,
    pub token: String,
    pub service_code: EnumService,
    pub device_id: String,
    pub device_os: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FooRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FooResponse {
    pub foo: bool,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersRequest {
    pub offset: i32,
    pub limit: i32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersResponse {
    pub users: Vec<ListUsersResponseRow>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersResponseRow {
    pub user_public_id: i64,
    pub username: String,
    pub email: String,
    pub created_at: i32,
    pub updated_at: i32,
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
