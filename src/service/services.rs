include!("auth/pg_func.rs");
include!("auth/endpoints.rs");
pub fn get_services() -> Vec<Service> {
    vec![
        Service::new("auth", 1, get_auth_endpoints()),
        Service::new("user", 2, vec![]),
        Service::new("admin", 3, vec![]),
    ]
}

pub fn get_proc_functions() -> Vec<ProceduralFunction> {
    vec![get_auth_pg_func()].concat()
}
