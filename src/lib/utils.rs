use convert_case::{Case, Casing};
use eyre::*;
use model::endpoint::EndpointSchema;
use serde::Serialize;
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_log_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as _
}

pub fn get_conn_id() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as _
}
pub fn encode_header<T: Serialize>(v: T, schema: EndpointSchema) -> Result<String> {
    let mut s = String::new();
    write!(s, "0{}", schema.name.to_ascii_lowercase())?;
    let v = serde_json::to_value(&v)?;

    for (i, f) in schema.parameters.iter().enumerate() {
        let key = f.name.to_case(Case::Camel);
        write!(
            s,
            ", {}{}",
            i + 1,
            v.get(&key)
                .with_context(|| format!("key: {}", key))?
                .to_string()
                .replace("\"", "")
        )?;
    }
    Ok(s)
}
