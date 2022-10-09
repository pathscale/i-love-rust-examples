pub mod db;
pub mod endpoints;
pub mod method;
pub mod parsetools;

pub type Payload = gluesql::core::executor::Payload;
pub type Labels = Vec<String>;
pub type Row = Vec<gluesql::core::data::Value>;
pub type Value = gluesql::core::data::Value;
