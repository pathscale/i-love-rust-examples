use futures::future::BoxFuture;
use futures::FutureExt;
use gen::database::{DbClient, FunAuthAuthorizeReq};
use gen::model::EnumService;
use lib::ws::{AuthController, Connection};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

pub struct LoginAuthController {
    pub db: DbClient,
}

impl AuthController for LoginAuthController {
    // 0login, 1pepe_pablo, 2AHJQ6X1H68SK8D9P6WW0, 31, 424787297130491616, 5android
    fn auth(&self, header: String, mut conn: Connection) -> BoxFuture<'static, Result<Connection>> {
        let db = self.db.clone();
        async move {
            let splits = header
                .split(",")
                .map(|x| x.trim())
                .map(|x| (&x[..1], &x[1..]))
                .collect::<HashMap<&str, &str>>();
            let method = splits
                .get("0")
                .ok_or_else(|| eyre!("Could not find method"))?;
            if *method == "login" {
                // "role": "user",
                // "userPublicId": public_id,
                // "token": token,
                // "deviceId": parse.unquote(device_id),
                // "deviceOS": parse.unquote(device_os),
                let username = splits
                    .get("1")
                    .ok_or_else(|| eyre!("Could not find 1username"))?;
                let token = splits
                    .get("2")
                    .ok_or_else(|| eyre!("Could not find 2token"))?;
                let service = splits
                    .get("3")
                    .ok_or_else(|| eyre!("Could not find 3service"))?;

                let device_id = splits
                    .get("4")
                    .ok_or_else(|| eyre!("Could not find 4device_id"))?;

                let device_os = splits
                    .get("5")
                    .ok_or_else(|| eyre!("Could not find device"))?;
                info!(
                    "Logging in: {} <token> {} {} {}",
                    username, service, device_id, device_os
                );
                let auth_data = db
                    .fun_auth_authorize(FunAuthAuthorizeReq {
                        username: username.to_string(),
                        token: Uuid::from_str(token)?,
                        service: EnumService::User,
                        device_id: device_id.to_string(),
                        device_os: device_os.to_string(),
                        ip_address: conn.address,
                    })
                    .await?;
                let auth_data = &auth_data.rows[0];

                conn.user_id = auth_data.user_id as _;
                conn.role = auth_data.role as _;

                Ok(conn)
            } else {
                bail!("Could not process method {}", method)
            }
        }
        .boxed()
    }
}
