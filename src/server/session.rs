use {
    super::{
        Http,
        Server,
        SessionStore,
    },
    crate::{
        debug,
        message::Request,
        utils::{
            generate_session_id,
            get_current_timestamp,
            get_session_id,
            HTTP,
        },
    },
    std::{
        collections::HashMap,
        time::Duration,
    },
};

impl Http {
    pub fn new(timeout_minutes: u64) -> Self {
        Self {
            session_store: SessionStore::new(timeout_minutes),
        }
    }

    pub fn has_valid_session(&mut self, req: &Request) -> bool {
        if let Some(cookie) = req.headers.get("Cookie") {
            if let Some(session_id) = get_session_id(cookie) {
                return self
                    .session_store
                    .validate_session(&session_id.clone());
            }
        }
        false
    }
}

impl SessionStore {
    pub fn new(timeout_minutes: u64) -> Self {
        Self {
            sessions: HashMap::new(),
            timeout:  Duration::from_secs(timeout_minutes * 60),
        }
    }

    pub fn create_session(&mut self) -> String {
        self.clean();
        let session_id = generate_session_id();
        let timestamp = get_current_timestamp();
        self.sessions
            .insert(session_id.clone(), timestamp);
        session_id
    }

    pub fn validate_session(&mut self, session_id: &str) -> bool {
        self.clean();
        self.sessions
            .get(session_id)
            .map_or(false, |&timestamp| {
                get_current_timestamp() - timestamp < self.timeout.as_secs()
            })
    }

    pub fn clean(&mut self) {
        let current_time = get_current_timestamp();
        self.sessions
            .retain(|_, &mut timestamp| current_time - timestamp < self.timeout.as_secs());
    }
}

impl Server {
    pub(super) fn check_session(&self, path: &str, req: &Request) -> bool {
        println!("===================================================");
        println!("Checking session for path: {}", path);
        println!("===================================================");
        let reformat_path = self.reformat_path(path);
        if !self.get_session(&reformat_path) {
            debug!(path);
            println!("Path doesn't require session");
            return true;
        }

        match HTTP.write() {
            Ok(mut http) => {
                let has_session = http.has_valid_session(req);
                println!("Session validation result: {}", has_session);
                has_session
            }
            Err(e) => {
                println!("Failed to access HTTP: {:?}", e);
                false
            }
        }
    }

    pub fn get_session(&self, path: &str) -> bool {
        for route in self.routes() {
            if let Some(path_route) = &route.path {
                if path_route == path {
                    return route.check_session();
                }
            }
        }
        false
    }
}
