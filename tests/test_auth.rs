mod tools;
use eyre::*;
use model::endpoint::*;
use serde::*;
use tools::*;

include!("../src/service/auth/endpoints.rs");

#[tokio::test]
async fn test_login() -> Result<()> {
    let mut client = get_ws_client().await?;
    let res: AuthLoginResp = client
        .request(
            endpoint_auth_login().code,
            AuthLoginReq {
                username: "jack".to_string(),
                password: "123456".to_string(),
                service_code: 0,
                device_id: "".to_string(),
                device_os: "".to_string(),
            },
        )
        .await?;
    println!("{:?}", res);
    Ok(())
}
