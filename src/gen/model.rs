use serde::*;
use strum_macros::{Display, EnumString};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumString, Display)]
pub enum EnumRole {
    #[strum(to_string = "guest")]
    Guest = 0,
    #[strum(to_string = "user")]
    User = 1,
    #[strum(to_string = "admin")]
    Admin = 2,
    #[strum(to_string = "developer")]
    Developer = 3,
}
impl std::convert::TryFrom<i32> for EnumRole {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(EnumRole::Guest),
            1 => Ok(EnumRole::User),
            2 => Ok(EnumRole::Admin),
            3 => Ok(EnumRole::Developer),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumString, Display)]
pub enum EnumRecoveryQuestionCategory {
    #[strum(to_string = "childhood")]
    Childhood = 0,
    #[strum(to_string = "education")]
    Education = 1,
    #[strum(to_string = "family")]
    Family = 2,
    #[strum(to_string = "favorite")]
    Favorite = 3,
    #[strum(to_string = "first")]
    First = 4,
    #[strum(to_string = "personal")]
    Personal = 5,
    #[strum(to_string = "pet")]
    Pet = 6,
    #[strum(to_string = "work")]
    Work = 7,
    #[strum(to_string = "historical")]
    Historical = 8,
}
impl std::convert::TryFrom<i32> for EnumRecoveryQuestionCategory {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(EnumRecoveryQuestionCategory::Childhood),
            1 => Ok(EnumRecoveryQuestionCategory::Education),
            2 => Ok(EnumRecoveryQuestionCategory::Family),
            3 => Ok(EnumRecoveryQuestionCategory::Favorite),
            4 => Ok(EnumRecoveryQuestionCategory::First),
            5 => Ok(EnumRecoveryQuestionCategory::Personal),
            6 => Ok(EnumRecoveryQuestionCategory::Pet),
            7 => Ok(EnumRecoveryQuestionCategory::Work),
            8 => Ok(EnumRecoveryQuestionCategory::Historical),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumString, Display)]
pub enum EnumService {
    #[strum(to_string = "auth")]
    Auth = 1,
    #[strum(to_string = "user")]
    User = 2,
    #[strum(to_string = "admin")]
    Admin = 3,
}
impl std::convert::TryFrom<i32> for EnumService {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(EnumService::Auth),
            2 => Ok(EnumService::User),
            3 => Ok(EnumService::Admin),
            _ => Err(()),
        }
    }
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
