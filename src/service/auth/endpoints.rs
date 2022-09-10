use model::endpoint::*;
use model::types::{Field, Type};
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthSignupRequest {
    pub username: String,
    pub password: String,
    pub agreed_tos: bool,
    pub agreed_privacy: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthSignupResponse {
    pub username: String,
    pub user_public_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginReq {
    pub username: String,
    pub password: String,
    pub service_code: i32,
    pub device_id: String,
    pub device_os: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginResp {
    pub username: String,
    pub user_public_id: i64,
    pub user_token: String,
    pub admin_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthAuthorizeReq {
    pub username: String,
    pub token: String,
    pub service_code: i32,
    pub device_id: String,
    pub device_os: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthAuthorizeResp {
    pub success: bool,
}

pub fn endpoint_auth_signup() -> EndpointSchema {
    EndpointSchema::new(
        "Signup",
        10010,
        vec![
            Field::new("username", Type::Text),
            Field::new("password", Type::Text),
            Field::new("agreed_tos", Type::Boolean),
            Field::new("agreed_privacy", Type::Boolean),
        ],
        vec![
            Field::new("username", Type::Text),
            Field::new("user_public_id", Type::BigInt),
        ],
    )
}
pub fn endpoint_auth_login() -> EndpointSchema {
    EndpointSchema::new(
        "Login",
        10020,
        vec![
            Field::new("username", Type::Text),
            Field::new("password", Type::Text),
            Field::new("service_code", Type::Int),
            Field::new("device_id", Type::Text),
            Field::new("device_os", Type::Text),
        ],
        vec![],
    )
}
pub fn endpoint_auth_authorize() -> EndpointSchema {
    EndpointSchema::new(
        "Authorize",
        10030,
        vec![
            Field::new("username", Type::Text),
            Field::new("token", Type::Text),
            Field::new("service_code", Type::Int),
            Field::new("device_id", Type::Text),
            Field::new("device_os", Type::Text),
        ],
        vec![],
    )
}

pub fn get_auth_endpoints() -> Vec<EndpointSchema> {
    vec![
        endpoint_auth_login(),
        endpoint_auth_signup(),
        endpoint_auth_authorize(),
    ]
}
