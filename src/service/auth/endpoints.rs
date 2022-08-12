pub fn endpoint_auth_foo() -> WsEndpointSchema {
    WsEndpointSchema::new("foo", 10010, vec![], vec![])
}
pub fn get_auth_endpoints() -> Vec<WsEndpointSchema> {
    vec![endpoint_auth_foo()]
}
