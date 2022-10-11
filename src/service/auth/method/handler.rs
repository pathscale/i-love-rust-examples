use eyre::*;
use gen::database::*;
use gen::model::*;
use lib::database::LocalDbClient;
use lib::handler::RequestHandler;
use lib::id_gen::ConcurrentSnowflake;
use lib::toolbox::*;
use lib::ws::*;
use reqwest::StatusCode;
use sha2::{Digest, Sha256};
use std::str::FromStr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use uuid::Uuid;

use super::repository;
pub struct SignupHandler {
    pub id_gen: ConcurrentSnowflake,
}

impl RequestHandler for SignupHandler {
    type Request = SignupRequest;
    type Response = SignupResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: LocalDbClient = toolbox.get_db();
        let mut snowflake = self.id_gen.clone();
        toolbox.spawn_response(ctx, async move {
            let public_id = snowflake.gen()?;
            let salt = Uuid::new_v4();
            let password_hash = hash_password(&req.password, salt.as_bytes())?;
            let username = req.username.trim().to_ascii_lowercase();

            let agreed_tos = req.agreed_tos;
            let agreed_privacy = req.agreed_privacy;

            if !agreed_tos {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("terms of service not consented"),
                ));
            }
            if !agreed_privacy {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("privacy policy not consented"),
                ));
            }
            if username == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("empty username not allowed"),
                ));
            }
            if req.password.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("empty password not allowed"),
                ));
            }
            if req.email.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid email"),
                ));
            }
            if req.phone.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid phone number"),
                ));
            }

            let req = FunAuthSignupReq {
                public_id,
                username: username.clone(),
                email: req.email,
                phone: req.phone,
                password_hash,
                password_salt: salt.as_bytes().to_vec(),
                age: 0,
                preferred_language: "".to_string(),
                agreed_tos,
                agreed_privacy,
                ip_address: conn.address.clone(),
            };

            repository::fun_auth_signup(&db, snowflake, req).await?;

            Ok(SignupResponse {
                username: username,
                user_public_id: public_id,
            })
        });
    }
}

pub struct LoginHandler {
    pub id_gen: ConcurrentSnowflake,
}

impl RequestHandler for LoginHandler {
    type Request = LoginRequest;
    type Response = LoginResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: LocalDbClient = toolbox.get_db();
        let snowflake = self.id_gen.clone();
        toolbox.spawn_response(ctx, async move {
            let username = req.username.trim().to_ascii_lowercase();
            let service_code = req.service_code;
            let password = req.password;

            if username == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid username"),
                ));
            }
            if password.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid password"),
                ));
            }
            if req.device_id.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid device id"),
                ));
            }
            if req.device_os.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid device os"),
                ));
            }

            let salt = &repository::fun_auth_get_password_salt(
                &db,
                FunAuthGetPasswordSaltReq {
                    username: username.clone(),
                },
            )
            .await?
            .rows[0]
                .salt;

            let mut hasher = Sha256::new();

            hasher.update(password.as_bytes());
            hasher.update(salt.as_slice());
            let password_hash = hasher.finalize().to_vec();

            let auth_data = &repository::fun_auth_authenticate(
                &db,
                snowflake,
                FunAuthAuthenticateReq {
                    username: username.clone(),
                    password_hash: password_hash.clone(),
                    service_code: service_code as _,
                    device_id: req.device_id.clone(),
                    device_os: req.device_os.clone(),
                    ip_address: conn.address.clone(),
                },
            )
            .await?
            .rows[0];

            let user_token = Uuid::new_v4();
            let admin_token = Uuid::new_v4();

            repository::fun_auth_set_token(
                &db,
                FunAuthSetTokenReq {
                    user_id: auth_data.user_id,
                    user_token: user_token.clone(),
                    admin_token: admin_token.clone(),
                    service_code: service_code as _,
                },
            )
            .await?;

            Ok(LoginResponse {
                username: username.clone(),
                user_public_id: auth_data.user_public_id,
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

pub struct AuthorizeHandler {
    pub accept_service: EnumService,
    pub id_gen: ConcurrentSnowflake,
}
impl RequestHandler for AuthorizeHandler {
    type Request = AuthorizeRequest;
    type Response = AuthorizeResponse;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: Self::Request,
    ) {
        let db: LocalDbClient = toolbox.get_db();
        let snowflake = self.id_gen.clone();
        let accept_srv = self.accept_service;
        toolbox.spawn_response(ctx, async move {
            let username = req.username.trim().to_ascii_lowercase();
            if req.service_code != accept_srv {
                bail!(CustomError::new(
                    StatusCode::FORBIDDEN,
                    format!(
                        "Invalid service, only {:?} {} permitted",
                        accept_srv, accept_srv as u32
                    ),
                ));
            }
            if username == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid username"),
                ));
            }
            if req.token.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid token"),
                ));
            }
            if req.device_id.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid device id"),
                ));
            }
            if req.device_os.trim() == "" {
                bail!(CustomError::new(
                    StatusCode::BAD_REQUEST,
                    format!("invalid device os"),
                ));
            }

            let auth_data = &repository::fun_auth_authorize(
                &db,
                snowflake,
                FunAuthAuthorizeReq {
                    username: username,
                    token: Uuid::from_str(&req.token)?,
                    service: req.service_code,
                    device_id: req.device_id,
                    device_os: req.device_os,
                    ip_address: conn.address,
                },
            )
            .await?
            .rows[0];

            conn.user_id
                .store(auth_data.user_id as _, Ordering::Relaxed);
            conn.role.store(auth_data.role as _, Ordering::Relaxed);
            Ok(AuthorizeResponse { success: true })
        })
    }
}
