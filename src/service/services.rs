use model::service::*;
use model::types::*;

#[path = "auth/endpoints.rs"]
mod auth_endpoints;
#[path = "auth/repo_func.rs"]
mod auth_repo_func;

#[path = "user/endpoints.rs"]
mod user_endpoints;

#[path = "user/repo_func.rs"]
mod user_repo_func;

#[path = "admin/endpoints.rs"]
mod admin_endpoints;

#[path = "admin/repo_func.rs"]
mod admin_repo_func;

pub fn get_services() -> Vec<Service> {
    vec![
        Service::new("auth", 1, auth_endpoints::get_auth_endpoints()),
        Service::new("user", 2, user_endpoints::get_user_endpoints()),
        Service::new("admin", 3, admin_endpoints::get_admin_endpoints()),
    ]
}

pub fn get_repo_functions() -> Vec<RepositoryFunction> {
    vec![
        auth_repo_func::get_auth_repo_func(),
        user_repo_func::get_user_repo_func(),
        admin_repo_func::get_admin_repo_func(),
    ]
    .concat()
}
