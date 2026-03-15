use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

#[cfg(target_os = "macos")]
use {
    libc::{
        c_long,
        time_t,
        timespec,
    },
    std::time::Duration,
};

#[cfg(target_os = "macos")]
pub fn timeout(timeout_in_ms: u64) -> *const timespec {
    let duration = Duration::from_millis(timeout_in_ms);
    let secs = duration.as_secs() as time_t;
    let nanos = duration.subsec_nanos() as c_long;

    &timespec {
        tv_sec:  secs,
        tv_nsec: nanos,
    }
}

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
