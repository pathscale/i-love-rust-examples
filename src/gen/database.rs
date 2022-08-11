use eyre::*;
use tokio_postgres::*;
pub struct DatabaseClient {
    client: Client,
}
impl DatabaseClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

pub struct FunAuthAddReq {
    pub a: i32,
    pub b: i32,
}
pub struct FunAuthAddRespRow {
    pub sum: i32,
}
pub struct FunAuthAddResp {
    pub rows: Vec<FunAuthAddRespRow>,
}
impl DatabaseClient {
    pub async fn fun_auth_add(&self, req: FunAuthAddReq) -> Result<FunAuthAddResp> {
        let rows = self
            .client
            .query(
                "SELECT * FROM api.fun_auth_add(a => 0::int,b => 1::int);",
                &[&req.a, &req.b],
            )
            .await?;
        let mut resp = FunAuthAddResp {
            rows: Vec::with_capacity(rows.len()),
        };
        for row in rows {
            let r = FunAuthAddRespRow {
                sum: row.try_get(1)?,
            };
            resp.rows.push(r);
        }
        Ok(resp)
    }
}
