pub mod tools;
use crate::endpoints::*;
use eyre::*;
use gen::model::EnumService;
use tools::*;

#[path = "../src/service/auth/endpoints.rs"]
pub mod endpoints;
#[tokio::test]
async fn test_authorize() -> Result<()> {
    let mut client = get_ws_auth_client(
        "0login, 1pepe_pablo, 2AHJQ6X1H68SK8D9P6WW0, 32, 424787297130491616, 5android",
    )
    .await?;
    let res: AuthLoginResp = client.recv_resp().await?;

    let mut client = get_ws_user_client(
        &format!(
            "0authorize, 1{}, 2{}, 3{}, 4{}, 5{}",
            res.username,
            res.user_token,
            EnumService::User as u32,
            "24787297130491616",
            "android"
        )
        .to_string(),
    )
    .await?;
    let res: AuthAuthorizeResp = client.recv_resp().await?;
    println!("{:?}", res);
    Ok(())
}
