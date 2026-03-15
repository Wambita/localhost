use {
    super::AppErr,
    std::{
        error,
        fmt::Display,
        io,
        net::AddrParseError,
        panic::Location,
    },
};

impl Display for AppErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let loc = Location::caller();
        write!(f, "[{}:{}] - ", loc.file(), loc.line())?;
        match self {
            Self::Custom(msg) => writeln!(f, "Error: {msg}."),
            Self::DeserializeTOML(e) => writeln!(f, "TOML: {e}."),
            Self::SerDeJSON(e) => writeln!(f, "JSON: {e}."),
            Self::NonBlocking(e) => writeln!(f, "Non-blocking: {e}."),
            Self::ParseAddr(e) => writeln!(f, "Address Parsing: {e}."),
            Self::TmplNotFound(e) => writeln!(f, "Template: {e}."),
            Self::Buffering => writeln!(f, "Incomplete Buffer"),
            Self::IncompleteRequest => writeln!(f, "Incomplete Request"),
            Self::TooLarge => writeln!(f, "Request too large"),
            Self::NotFound(e) => writeln!(f, "Not Found: {e}."),
            Self::Other(e) => writeln!(f, "ERROR: {e}."),
            Self::EmptyBuffer => writeln!(f, "Empty buffer"),
            Self::NoServer => writeln!(f, "No server to connect"),
            Self::NoClient => writeln!(f, "No client to connect"),
            Self::ExtNotFound => writeln!(f, "File extension not found"),
            Self::NoCGI => writeln!(f, "No Common Gateway Interface!"),
        }
    }
}

impl error::Error for AppErr {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::NonBlocking(e) => Some(e),
            Self::SerDeJSON(e) => Some(e),
            Self::DeserializeTOML(e) => Some(e),
            Self::ParseAddr(e) => Some(e),
            Self::Other(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for AppErr {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::WouldBlock => Self::NonBlocking(value),
            io::ErrorKind::NotFound => Self::NotFound(value),
            _ => Self::Other(value),
        }
    }
}

impl From<toml::de::Error> for AppErr {
    fn from(value: toml::de::Error) -> Self { Self::DeserializeTOML(value) }
}

impl From<AddrParseError> for AppErr {
    fn from(value: AddrParseError) -> Self { Self::ParseAddr(value) }
}

impl From<serde_json::Error> for AppErr {
    fn from(value: serde_json::Error) -> Self { Self::SerDeJSON(value) }
}

impl From<tera::Error> for AppErr {
    fn from(value: tera::Error) -> Self {
        match value.kind {
            tera::ErrorKind::TemplateNotFound(_) => Self::TmplNotFound(value),
            _ => Self::new(value.to_string().as_str()),
        }
    }
}

impl AppErr {
    pub fn new(msg: &str) -> Self { Self::Custom(msg.to_string()) }

    pub fn last_os_error() -> Self { Self::Other(io::Error::last_os_error()) }
}
