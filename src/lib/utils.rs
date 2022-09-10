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
