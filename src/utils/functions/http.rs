use {
    crate::message::{
        Method,
        Resource,
    },
    rand::{
        distributions::Alphanumeric,
        Rng,
    },
};

pub fn process_req_line(s: &str) -> (Method, Resource) {
    let mut words = s.split_whitespace();

    let method = words.next().unwrap();
    let resource = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
    )
}

pub fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':');
    let key = header_items
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    let value = header_items
        .collect::<Vec<&str>>()
        .join(":")
        .trim()
        .to_string();

    (key, value)
}

pub fn generate_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn get_session_id(cookie: &str) -> Option<String> {
    cookie
        .split(';')
        .find(|s| {
            s.trim()
                .starts_with("session_id=")
        })
        .map(|s| s.trim()["session_id=".len()..].to_string())
}

pub fn find_bytes(part: &[u8], need: &[u8]) -> Option<usize> {
    part.windows(need.len())
        .position(|windows| windows == need)
}
