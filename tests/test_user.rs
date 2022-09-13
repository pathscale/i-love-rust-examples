pub mod tools;
use crate::endpoints::*;
use eyre::*;
use gen::model::*;
use lib::utils::encode_header;
use tools::*;

#[path = "../src/service/auth/endpoints.rs"]
pub mod endpoints;
#[tokio::test]
async fn test_authorize() -> Result<()> {
    let mut client = get_ws_auth_client(&encode_header(
        &LoginRequest {
            username: "pepe_pablo".to_string(),
            password: "AHJQ6X1H68SK8D9P6WW0".to_string(),
            service_code: EnumService::User as _,
            device_id: "24787297130491616".to_string(),
            device_os: "android".to_string(),
        },
        endpoint_auth_login(),
    )?)
    .await?;
    let res: LoginResponse = client.recv_resp().await?;

    let mut client = get_ws_user_client(&encode_header(
        AuthorizeRequest {
            username: res.username,
            token: res.user_token,
            service_code: EnumService::User as _,
            device_id: "24787297130491616".to_string(),
            device_os: "android".to_string(),
        },
        endpoint_auth_authorize(),
    )?)
    .await?;
    let res: AuthorizeResponse = client.recv_resp().await?;
    println!("{:?}", res);
    Ok(())
}
