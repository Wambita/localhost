use {
    super::{
        Route,
        Server,
    },
    crate::{
        debug,
        utils::AppResult,
    },
    std::{
        io::ErrorKind,
        net::{
            SocketAddr,
            TcpListener,
        },
        str::FromStr,
    },
};

impl Route {
    pub fn path(&self) -> &str { self.path.as_ref().unwrap() }

    pub fn check_session(&self) -> bool { self.need_session.unwrap() }
}

impl Server {
    pub fn host(&self) -> &str { self.host.as_ref().unwrap() }

    pub fn ip(&self) -> &str { self.ip.as_ref().unwrap() }

    pub fn ports(&self) -> &Vec<usize> { self.ports.as_ref().unwrap() }

    pub fn root(&self) -> &str { self.root.as_ref().unwrap() }

    pub fn listing(&self) -> bool { self.listing.unwrap() }

    pub fn routes(&self) -> &Vec<Route> { self.routes.as_ref().unwrap() }

    pub fn listeners(&self) -> AppResult<Vec<TcpListener>> {
        let mut listeners = vec![];
        let ip = self.ip();

        for port in self.ports() {
            let address = SocketAddr::from_str(&format!("{ip}:{port}"))?;
            match TcpListener::bind(debug!(address)) {
                Ok(listener) => listeners.push(listener),
                Err(e) if e.kind() == ErrorKind::AddrInUse => {
                    debug!(e);
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(listeners)
    }
}
