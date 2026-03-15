mod http;
mod time;

#[cfg(target_os = "macos")]
pub use time::timeout;
pub use {
    http::{
        find_bytes,
        generate_session_id,
        get_session_id,
        process_header_line,
        process_req_line,
    },
    time::get_current_timestamp,
};
