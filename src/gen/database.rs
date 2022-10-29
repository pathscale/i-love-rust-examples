
use crate::model::*;
pub struct FunAuthSignupReq {
    pub public_id: i64,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub age: i32,
    pub preferred_language: String,
    pub agreed_tos: bool,
    pub agreed_privacy: bool,
    pub ip_address: std::net::IpAddr,
}
pub struct FunAuthSignupRespRow {
    pub user_id: i64,
}
pub struct FunAuthSignupResp {
    pub rows: Vec<FunAuthSignupRespRow>,
}
pub struct FunAuthAuthenticateReq {
    pub username: String,
    pub password_hash: Vec<u8>,
    pub service_code: i32,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}
pub struct FunAuthAuthenticateRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
}
pub struct FunAuthAuthenticateResp {
    pub rows: Vec<FunAuthAuthenticateRespRow>,
}
pub struct FunAuthGetPasswordSaltReq {
    pub username: String,
}
pub struct FunAuthGetPasswordSaltRespRow {
    pub salt: Vec<u8>,
}
pub struct FunAuthGetPasswordSaltResp {
    pub rows: Vec<FunAuthGetPasswordSaltRespRow>,
}
pub struct FunAuthSetTokenReq {
    pub user_id: i64,
    pub user_token: uuid::Uuid,
    pub admin_token: uuid::Uuid,
    pub service_code: i32,
}
pub struct FunAuthSetTokenRespRow {}
pub struct FunAuthSetTokenResp {
    pub rows: Vec<FunAuthSetTokenRespRow>,
}
pub struct FunAuthAuthorizeReq {
    pub username: String,
    pub token: uuid::Uuid,
    pub service: EnumService,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}
pub struct FunAuthAuthorizeRespRow {
    pub user_id: i64,
    pub role: EnumRole,
}
pub struct FunAuthAuthorizeResp {
    pub rows: Vec<FunAuthAuthorizeRespRow>,
}
pub struct FunAuthChangePasswordReq {
    pub username: String,
    pub old_password_hash: Vec<u8>,
    pub new_password_hash: Vec<u8>,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}
pub struct FunAuthChangePasswordRespRow {}
pub struct FunAuthChangePasswordResp {
    pub rows: Vec<FunAuthChangePasswordRespRow>,
}
pub struct FunGetRecoveryQuestionDataReq {}
pub struct FunGetRecoveryQuestionDataRespRow {
    pub question_id: i64,
    pub content: String,
    pub category: EnumRecoveryQuestionCategory,
}
pub struct FunGetRecoveryQuestionDataResp {
    pub rows: Vec<FunGetRecoveryQuestionDataRespRow>,
}
pub struct FunAuthSetRecoveryQuestionsReq {
    pub user_id: i64,
    pub question_ids: Vec<i64>,
    pub answers: Vec<i32>,
}
pub struct FunAuthSetRecoveryQuestionsRespRow {}
pub struct FunAuthSetRecoveryQuestionsResp {
    pub rows: Vec<FunAuthSetRecoveryQuestionsRespRow>,
}
pub struct FunAuthBasicAuthenticateReq {
    pub username: String,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}
pub struct FunAuthBasicAuthenticateRespRow {
    pub user_id: std::net::IpAddr,
}
pub struct FunAuthBasicAuthenticateResp {
    pub rows: Vec<FunAuthBasicAuthenticateRespRow>,
}
pub struct FunAuthGetRecoveryQuestionsReq {
    pub user_id: i64,
}
pub struct FunAuthGetRecoveryQuestionsRespRow {
    pub question_id: i64,
    pub question: String,
}
pub struct FunAuthGetRecoveryQuestionsResp {
    pub rows: Vec<FunAuthGetRecoveryQuestionsRespRow>,
}
pub struct FunSubmitRecoveryAnswersReq {
    pub user_id: i64,
    pub question_ids: Vec<i64>,
    pub answers: Vec<String>,
    pub password_reset_token: uuid::Uuid,
    pub token_valid: i32,
}
pub struct FunSubmitRecoveryAnswersRespRow {}
pub struct FunSubmitRecoveryAnswersResp {
    pub rows: Vec<FunSubmitRecoveryAnswersRespRow>,
}
pub struct FunAuthResetPasswordReq {
    pub user_id: i64,
    pub new_password_hash: Vec<u8>,
    pub new_password_salt: Vec<u8>,
    pub reset_token: uuid::Uuid,
}
pub struct FunAuthResetPasswordRespRow {}
pub struct FunAuthResetPasswordResp {
    pub rows: Vec<FunAuthResetPasswordRespRow>,
}
pub struct FunAdminListUsersReq {
    pub offset: i32,
    pub limit: i32,
}
pub struct FunAdminListUsersRespRow {
    pub user_id: i64,
    pub user_public_id: i64,
    pub email: String,
    pub username: String,
    pub role: EnumRole,
    pub updated_at: i64,
    pub created_at: i64,
}
pub struct FunAdminListUsersResp {
    pub rows: Vec<FunAdminListUsersRespRow>,
}
pub struct FunAdminAssignRoleReq {
    pub operator_user_id: i64,
    pub user_public_id: i64,
    pub new_role: EnumRole,
}
pub struct FunAdminAssignRoleRespRow {}
pub struct FunAdminAssignRoleResp {
    pub rows: Vec<FunAdminAssignRoleRespRow>,
}
