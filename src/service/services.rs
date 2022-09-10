use model::service::*;
use model::types::*;

#[path = "auth/endpoints.rs"]
mod auth_endpoints;
#[path = "auth/pg_func.rs"]
mod auth_pg_func;
#[path = "user/pg_func.rs"]
mod user_pg_func;

#[path = "user/endpoints.rs"]
mod user_endpoints;

pub fn get_services() -> Vec<Service> {
    vec![
        Service::new("auth", 1, auth_endpoints::get_auth_endpoints()),
        Service::new("user", 2, user_endpoints::get_user_endpoints()),
        Service::new("admin", 3, vec![]),
    ]
}

pub fn get_proc_functions() -> Vec<ProceduralFunction> {
    vec![
        auth_pg_func::get_auth_pg_func(),
        user_pg_func::get_user_pg_func(),
    ]
    .concat()
}
