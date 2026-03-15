mod cgi;
mod delete;
mod external;
mod file_system;
mod getters;
pub mod http;
mod router;
mod session;
mod upload;
mod validation;

pub use external::Http;
use {
    serde::{
        Deserialize,
        Serialize,
    },
    std::{
        collections::HashMap,
        time::Duration,
    },
};

#[derive(Serialize, Deserialize)]
pub struct Server {
    host:        Option<String>,
    ip:          Option<String>,
    ports:       Option<Vec<usize>>,
    root:        Option<String>,
    error_pages: Option<HashMap<String, String>>,
    listing:     Option<bool>,
    routes:      Option<Vec<Route>>,
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    path:         Option<String>,
    methods:      Option<Vec<String>>,
    default_file: Option<String>,
    need_session: Option<bool>,
    redirect:     Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct SessionStore {
    pub sessions: HashMap<String, u64>,
    pub timeout:  Duration,
}
