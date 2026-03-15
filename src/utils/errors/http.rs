use {
    super::{
        AppErr,
        HttpStatus,
    },
    std::io,
};

impl From<io::Error> for HttpStatus {
    fn from(_error: io::Error) -> Self { Self::from(500) }
}

impl From<AppErr> for HttpStatus {
    fn from(value: AppErr) -> Self {
        match value {
            AppErr::NoCGI | AppErr::ExtNotFound | AppErr::NotFound(_) | AppErr::TmplNotFound(_) => {
                Self::from(404)
            }
            AppErr::Custom(msg) => Self::from(msg),
            AppErr::TooLarge => Self::from(413),
            _ => Self::from(500),
        }
    }
}

impl From<String> for HttpStatus {
    fn from(message: String) -> Self {
        let msg = message.to_lowercase();

        let status_code = match msg {
            _ if msg.contains("ok") => 200,
            _ if msg.contains("moved permanently") => 301,
            _ if msg.contains("found") => 302,
            _ if msg.contains("see other") => 303,
            _ if msg.contains("bad request") => 400,
            _ if msg.contains("unauthorized") => 401,
            _ if msg.contains("forbidden") => 403,
            _ if msg.contains("not found") => 404,
            _ if msg.contains("method not allowed") => 405,
            _ if msg.contains("too large") => 413,
            _ => 500,
        };

        Self {
            status_code,
            message,
        }
    }
}

impl From<u16> for HttpStatus {
    fn from(status_code: u16) -> Self {
        let msg = match status_code {
            200 => "OK",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            413 => "Request Entity Too Large",
            _ => "Internal Server Error",
        };

        Self {
            status_code,
            message: msg.to_string(),
        }
    }
}
