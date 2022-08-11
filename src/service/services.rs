include!("auth/pg_func.rs");

pub fn get_services() -> Vec<Service> {
    vec![
        Service::new("auth", 1),
        Service::new("user", 2),
        Service::new("admin", 3),
    ]
}

pub fn get_proc_functions() -> Vec<ProceduralFunction> {
    vec![get_auth_pg_func()].concat()
}
