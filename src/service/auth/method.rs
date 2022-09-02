use eyre::*;

use crate::endpoints::*;
use gen::database::*;
use lib::toolbox::*;
use lib::ws::*;
use reqwest::StatusCode;
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub struct SignupHandler;

impl RequestHandler for SignupHandler {
    type Request = SignupRequest;
    type Response = SignupResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: WsRequestGeneric<Self::Request>,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            let public_id = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64;
            let salt = uuid::Uuid::new_v4();
            let password_hash = hash_password(&req.params.password, salt.as_bytes())?;
            let username = req.params.username.trim().to_ascii_lowercase();

            let agreed_tos = req.params.agreed_tos;
            let agreed_privacy = req.params.agreed_privacy;

            if !agreed_tos {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("You must agree to the terms of service"),
                ));
            }
            if !agreed_privacy {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("You must agree to the privacy policy"),
                ));
            }

            db.fun_auth_signup(FunAuthSignupReq {
                public_id,
                username: username.to_string(),
                password_hash,
                password_salt: salt.as_bytes().to_vec(),
                age: 0,
                preferred_language: "".to_string(),
                agreed_tos,
                agreed_privacy,
                ip_address: conn.address.clone(),
            })
            .await?;

            Ok(SignupResponse {
                username: username.to_string(),
                user_public_id: public_id,
            })
        });
    }
}

pub struct LoginHandler;

impl RequestHandler for LoginHandler {
    type Request = AuthLoginReq;
    type Response = AuthLoginResp;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: WsRequestGeneric<Self::Request>,
    ) {
        let db: DbClient = toolbox.get_db();
        toolbox.spawn_response(ctx, async move {
            let username = req.params.username.trim().to_ascii_lowercase();
            let service_code = req.params.service_code;
            let password = req.params.password;
            let data = db
                .fun_auth_get_password_salt(FunAuthGetPasswordSaltReq {
                    username: username.clone(),
                })
                .await?;
            let salt = data
                .rows
                .get(0)
                .ok_or(eyre!("No salt found for user"))?
                .salt
                .clone();

            let mut hasher = Sha256::new();

            hasher.update(password.as_bytes());
            hasher.update(salt.as_slice());
            let password_hash = hasher.finalize().to_vec();
            let data = db
                .fun_auth_authenticate(FunAuthAuthenticateReq {
                    username: username.clone(),
                    password_hash: password_hash.clone(),
                    password_salt: salt.clone(),
                    service_code,
                    device_id: req.params.device_id.clone(),
                    device_os: req.params.device_os.clone(),
                    ip_address: conn.address.clone(),
                })
                .await?;
            let row = data.rows.get(0).ok_or(eyre!("No rows found for user"))?;
            let user_token = uuid::Uuid::new_v4();
            let admin_token = uuid::Uuid::new_v4();
            db.fun_auth_set_token(FunAuthSetTokenReq {
                user_id: row.user_id,
                user_token: user_token.clone(),
                admin_token: admin_token.clone(),
                service_code,
            })
            .await?;
            Ok(AuthLoginResp {
                username: username.clone(),
                user_public_id: row.user_public_id,
                user_token: user_token.to_string(),
                admin_token: admin_token.to_string(),
            })
        })
    }
}
pub fn hash_password(password: &str, salt: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_ref());
    Ok(hasher.finalize().to_vec())
}
