pub use eyre::*;
use serde::de::DeserializeOwned;

pub struct JsonVerifier {

}

impl JsonVerifier {
    pub fn try_parse<T: DeserializeOwned>(&self, data: &[u8]) -> Result<T> {
        let v = serde_json::from_slice(data)?;
        Ok(v)
    }
}