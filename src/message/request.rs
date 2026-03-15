use {
    super::{
        Method,
        Request,
        Resource,
    },
    crate::{
        debug,
        utils::{
            process_header_line,
            process_req_line,
            AppErr,
            AppResult,
        },
    },
    std::{
        collections::HashMap,
        fmt,
    },
};

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "DELETE" => Self::DELETE,
            _ => Self::Uninitialized,
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::DELETE => writeln!(f, "DELETE"),
            Method::Uninitialized => write!(f, "Uninitialized Method"),
        }
    }
}

impl From<Vec<u8>> for Request {
    fn from(req_bytes: Vec<u8>) -> Self {
        let mut resource = Resource::Path("".to_string());
        let mut method = Method::Uninitialized;
        let mut headers = HashMap::new();
        let mut body = Vec::new(); // Body en Vec<u8>

        let delimiter = b"\r\n\r\n";
        if let Some(pos) = req_bytes
            .windows(delimiter.len())
            .position(|w| w == delimiter)
        {
            let (headers_part, body_part) = req_bytes.split_at(pos + delimiter.len());

            // Convertir la partie headers en String
            if let Ok(headers_str) = String::from_utf8(headers_part.to_vec()) {
                let mut lines = headers_str.lines();

                // Lire la première ligne (méthode + ressource)
                if let Some(first_line) = lines.next() {
                    if first_line.contains("HTTP") {
                        let (parsed_method, parsed_resource) = process_req_line(first_line);
                        method = parsed_method;
                        resource = parsed_resource;
                    }
                }

                // Lire le reste des en-têtes
                for line in lines {
                    if !line.is_empty() {
                        let (key, value) = process_header_line(line);
                        headers.insert(key, value);
                    }
                }
            }

            // Stocker le body en `Vec<u8>`
            body.extend_from_slice(body_part);
        }

        Self {
            resource,
            method,
            headers,
            body,
        }
    }
}

impl Request {
    pub fn get(req_bytes: &Vec<u8>, size_limit: u64) -> AppResult<Self> {
        let req_str = String::from_utf8_lossy(req_bytes).to_string();

        let content_len = req_str
            .lines()
            .find(|line| line.contains("Content-Length"))
            .map(|line| {
                line.split(":")
                    .nth(1)
                    .unwrap_or("0")
                    .trim()
                    .parse()
                    .unwrap_or(0)
            })
            .unwrap_or(0);

        if debug!(size_limit) < debug!(content_len as u64) {
            Err(debug!(AppErr::TooLarge))
        }
        else if debug!(req_bytes.len()) < content_len {
            Err(debug!(AppErr::IncompleteRequest))
        }
        else {
            Ok(req_bytes.clone().into())
        }
    }

    pub fn host(&self) -> &str {
        self.headers
            .get("Host")
            .map(|h| {
                h.split(':')
                    .nth(0)
                    .unwrap_or_default()
                    .trim()
            })
            .unwrap_or_default()
    }
}
