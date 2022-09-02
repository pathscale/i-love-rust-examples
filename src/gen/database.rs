use crate::model::*;
use eyre::*;
use lib::database::*;
#[derive(Clone)]
pub struct DbClient {
    client: SimpleDbClient,
}
impl DbClient {
    pub fn new(client: SimpleDbClient) -> Self {
        Self { client }
    }
}
impl From<SimpleDbClient> for DbClient {
    fn from(client: SimpleDbClient) -> Self {
        Self::new(client)
    }
}

pub struct FunAuthSignupReq {
    pub public_id: i64,
    pub username: String,
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
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_signup(&self, req: FunAuthSignupReq) -> Result<FunAuthSignupResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_signup(a_public_id => $1::bigint, a_username => $2::text, a_password_hash => $3::bytea, a_password_salt => $4::bytea, a_age => $5::int, a_preferred_language => $6::text, a_agreed_tos => $7::boolean, a_agreed_privacy => $8::boolean, a_ip_address => $9::inet);", &[&req.public_id, &req.username, &req.password_hash, &req.password_salt, &req.age, &req.preferred_language, &req.agreed_tos, &req.agreed_privacy, &req.ip_address]).await?;
        let mut resp = FunAuthSignupResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthSignupRespRow {
                user_id: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
pub struct FunAuthAuthenticateReq {
    pub username: String,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
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
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_authenticate(
        &self,
        req: FunAuthAuthenticateReq,
    ) -> Result<FunAuthAuthenticateResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_authenticate(a_username => $1::text, a_password_hash => $2::bytea, a_password_salt => $3::bytea, a_service_code => $4::int, a_device_id => $5::text, a_device_os => $6::text, a_ip_address => $7::inet);", &[&req.username, &req.password_hash, &req.password_salt, &req.service_code, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthAuthenticateResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthAuthenticateRespRow {
                user_id: row.try_get(0)?,
                user_public_id: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
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
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_get_password_salt(
        &self,
        req: FunAuthGetPasswordSaltReq,
    ) -> Result<FunAuthGetPasswordSaltResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_auth_get_password_salt(a_username => $1::text);",
                &[&req.username],
            )
            .await?;
        let mut resp = FunAuthGetPasswordSaltResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthGetPasswordSaltRespRow {
                salt: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
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
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_set_token(&self, req: FunAuthSetTokenReq) -> Result<FunAuthSetTokenResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_set_token(a_user_id => $1::bigint, a_user_token => $2::uuid, a_admin_token => $3::uuid, a_service_code => $4::int);", &[&req.user_id, &req.user_token, &req.admin_token, &req.service_code]).await?;
        let mut resp = FunAuthSetTokenResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthSetTokenRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
pub struct FunAuthAuthorizeReq {
    pub user_public_id: i64,
    pub token: uuid::Uuid,
    pub service: EnumService,
    pub device_id: String,
    pub device_os: String,
    pub ip_address: std::net::IpAddr,
}
pub struct FunAuthAuthorizeRespRow {
    pub user_id: std::net::IpAddr,
    pub role: EnumRole,
}
pub struct FunAuthAuthorizeResp {
    pub rows: Vec<FunAuthAuthorizeRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_authorize(
        &self,
        req: FunAuthAuthorizeReq,
    ) -> Result<FunAuthAuthorizeResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_authorize(a_user_public_id => $1::bigint, a_token => $2::uuid, a_service => $3::tbl.enum_service, a_device_id => $4::text, a_device_os => $5::text, a_ip_address => $6::inet);", &[&req.user_public_id, &req.token, &req.service, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthAuthorizeResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthAuthorizeRespRow {
                user_id: row.try_get(0)?,
                role: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
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
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_change_password(
        &self,
        req: FunAuthChangePasswordReq,
    ) -> Result<FunAuthChangePasswordResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_change_password(a_username => $1::text, a_old_password_hash => $2::bytea, a_new_password_hash => $3::bytea, a_device_id => $4::text, a_device_os => $5::text, a_ip_address => $6::inet);", &[&req.username, &req.old_password_hash, &req.new_password_hash, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthChangePasswordResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthChangePasswordRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
pub struct FunGetRecoveryQuestionDataReq {}
pub struct FunGetRecoveryQuestionDataRespRow {
    pub question_id: i32,
    pub content: String,
    pub category: EnumRecoveryQuestionCategory,
}
pub struct FunGetRecoveryQuestionDataResp {
    pub rows: Vec<FunGetRecoveryQuestionDataRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_get_recovery_question_data(
        &self,
        req: FunGetRecoveryQuestionDataReq,
    ) -> Result<FunGetRecoveryQuestionDataResp> {
        let rows = self
            .client
            .query("SELECT * FROM api.fun_get_recovery_question_data();", &[])
            .await?;
        let mut resp = FunGetRecoveryQuestionDataResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunGetRecoveryQuestionDataRespRow {
                question_id: row.try_get(0)?,
                content: row.try_get(1)?,
                category: row.try_get(2)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
pub struct FunAuthSetRecoveryQuestionsReq {
    pub user_id: i64,
    pub question_ids: Vec<i32>,
    pub answers: Vec<i32>,
}
pub struct FunAuthSetRecoveryQuestionsRespRow {}
pub struct FunAuthSetRecoveryQuestionsResp {
    pub rows: Vec<FunAuthSetRecoveryQuestionsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_set_recovery_questions(
        &self,
        req: FunAuthSetRecoveryQuestionsReq,
    ) -> Result<FunAuthSetRecoveryQuestionsResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_set_recovery_questions(a_user_id => $1::bigint, a_question_ids => $2::int[], a_answers => $3::int[]);", &[&req.user_id, &req.question_ids, &req.answers]).await?;
        let mut resp = FunAuthSetRecoveryQuestionsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthSetRecoveryQuestionsRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
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
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_basic_authenticate(
        &self,
        req: FunAuthBasicAuthenticateReq,
    ) -> Result<FunAuthBasicAuthenticateResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_basic_authenticate(a_username => $1::text, a_device_id => $2::text, a_device_os => $3::text, a_ip_address => $4::inet);", &[&req.username, &req.device_id, &req.device_os, &req.ip_address]).await?;
        let mut resp = FunAuthBasicAuthenticateResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthBasicAuthenticateRespRow {
                user_id: row.try_get(0)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
pub struct FunAuthGetRecoveryQuestionsReq {
    pub user_id: i64,
}
pub struct FunAuthGetRecoveryQuestionsRespRow {
    pub question_id: i32,
    pub question: String,
}
pub struct FunAuthGetRecoveryQuestionsResp {
    pub rows: Vec<FunAuthGetRecoveryQuestionsRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_get_recovery_questions(
        &self,
        req: FunAuthGetRecoveryQuestionsReq,
    ) -> Result<FunAuthGetRecoveryQuestionsResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_auth_get_recovery_questions(a_user_id => $1::bigint);",
                &[&req.user_id],
            )
            .await?;
        let mut resp = FunAuthGetRecoveryQuestionsResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthGetRecoveryQuestionsRespRow {
                question_id: row.try_get(0)?,
                question: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
pub struct FunSubmitRecoveryAnswersReq {
    pub user_id: i64,
    pub question_ids: Vec<i32>,
    pub answers: Vec<String>,
    pub password_reset_token: uuid::Uuid,
    pub token_valid: i32,
}
pub struct FunSubmitRecoveryAnswersRespRow {}
pub struct FunSubmitRecoveryAnswersResp {
    pub rows: Vec<FunSubmitRecoveryAnswersRespRow>,
}
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_submit_recovery_answers(
        &self,
        req: FunSubmitRecoveryAnswersReq,
    ) -> Result<FunSubmitRecoveryAnswersResp> {
        let rows = self.client.query("SELECT * FROM api.fun_submit_recovery_answers(a_user_id => $1::bigint, a_question_ids => $2::int[], a_answers => $3::text[], a_password_reset_token => $4::uuid, a_token_valid => $5::int);", &[&req.user_id, &req.question_ids, &req.answers, &req.password_reset_token, &req.token_valid]).await?;
        let mut resp = FunSubmitRecoveryAnswersResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunSubmitRecoveryAnswersRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
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
impl DbClient {
    #[allow(unused_variables)]
    pub async fn fun_auth_reset_password(
        &self,
        req: FunAuthResetPasswordReq,
    ) -> Result<FunAuthResetPasswordResp> {
        let rows = self.client.query("SELECT * FROM api.fun_auth_reset_password(a_user_id => $1::bigint, a_new_password_hash => $2::bytea, a_new_password_salt => $3::bytea, a_reset_token => $4::uuid);", &[&req.user_id, &req.new_password_hash, &req.new_password_salt, &req.reset_token]).await?;
        let mut resp = FunAuthResetPasswordResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthResetPasswordRespRow {};
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
