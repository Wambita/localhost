#[macro_use]
mod macros;
mod errors;
mod functions;
mod globals;
// mod chunk;

#[cfg(target_os = "macos")]
pub(super) use functions::timeout;
pub(super) use {
    errors::{
        AppErr,
        AppResult,
        HttpResult,
        HttpStatus,
    },
    functions::{
        find_bytes,
        generate_session_id,
        get_current_timestamp,
        get_session_id,
        process_header_line,
        process_req_line,
    },
    globals::{
        BOUNDARY_REGEX,
        CONTENT_DISPOSITION_REGEX,
        HTTP,
        TEMPLATES,
        TIMEOUT,
    },
};
