use super::{
    Route,
    Server,
};

impl Route {
    pub fn has_valid_config(&self) -> bool {
        self.path.is_some() && self.methods.is_some() && self.need_session.is_some()
    }
}

impl Server {
    pub fn has_valid_config(&self) -> bool {
        self.host.is_some()
            && self.ip.is_some()
            && self.ports.is_some()
            && self.root.is_some()
            && self.listing.is_some()
            && self.routes.is_some()
            && self
                .routes
                .as_ref()
                .unwrap()
                .iter()
                .all(|route| route.has_valid_config())
    }

    // pub fn format_root(&mut self) {
    //     match self.root() {
    //        "/" =>  self.root = Some(""),
    //     }
    // }
}
