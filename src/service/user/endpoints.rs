use model::endpoint::*;
use model::types::*;

pub fn endpoint_user_foo() -> EndpointSchema {
    EndpointSchema::new("Foo", 20010, vec![], vec![Field::new("foo", Type::Boolean)])
}
pub fn get_user_endpoints() -> Vec<EndpointSchema> {
    vec![endpoint_user_foo()]
}
