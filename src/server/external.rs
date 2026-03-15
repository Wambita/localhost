use {
    super::SessionStore,
    serde::{
        Deserialize,
        Serialize,
    },
    std::collections::HashMap,
};

#[derive(Serialize, Deserialize)]
pub struct Http {
    pub session_store: SessionStore,
}

type Interpreters = HashMap<String, String>;

/// Common Gateway Interface
#[derive(Serialize, Deserialize)]
pub struct Cgi {
    interpreters: Option<Interpreters>,
}

pub struct Upload;
