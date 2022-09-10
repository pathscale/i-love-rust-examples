pub mod tools;
use crate::endpoints::*;
use eyre::*;
use tools::*;

#[path = "../src/service/auth/endpoints.rs"]
pub mod endpoints;
#[tokio::test]
async fn test_bad_login() -> Result<()> {
    let mut client = get_ws_auth_client("").await?;
    let res: AuthLoginResp = client.recv_resp().await?;
    println!("{:?}", res);
    Ok(())
}

#[tokio::test]
async fn test_login() -> Result<()> {
    let mut client = get_ws_auth_client(
        "0login, 1pepe_pablo, 2AHJQ6X1H68SK8D9P6WW0, 32, 424787297130491616, 5android",
    )
    .await?;
    let res: AuthLoginResp = client.recv_resp().await?;
    println!("{:?}", res);
    Ok(())
}

#[tokio::test]
async fn test_signup() -> Result<()> {
    let mut client =
        get_ws_auth_client("0signup, 1pepe_pablo, 2AHJQ6X1H68SK8D9P6WW0, 3true, 4true").await?;
    let res: AuthSignupResponse = client.recv_resp().await?;
    println!("{:?}", res);
    Ok(())
}
