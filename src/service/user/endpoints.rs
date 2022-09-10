use model::endpoint::*;
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FooRequest {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FooResponse {
    pub foo: bool,
}

pub fn endpoint_user_foo() -> EndpointSchema {
    EndpointSchema::new("Foo", 20010, vec![], vec![])
}
pub fn get_user_endpoints() -> Vec<EndpointSchema> {
    vec![endpoint_user_foo()]
}
