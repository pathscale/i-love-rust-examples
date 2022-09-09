#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
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

pub fn endpoint_auth_signup() -> EndpointSchema {
    EndpointSchema::new("Signup", 10010, vec![], vec![])
}
pub fn endpoint_auth_login() -> EndpointSchema {
    EndpointSchema::new("Login", 10020, vec![], vec![])
}
pub fn get_auth_endpoints() -> Vec<EndpointSchema> {
    vec![endpoint_auth_login(), endpoint_auth_signup()]
}
