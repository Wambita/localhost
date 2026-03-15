use {
    super::{
        Headers,
        Response,
    },
    crate::utils::{
        AppErr,
        HttpStatus,
        TEMPLATES,
    },
    std::collections::HashMap,
    tera::Context,
};

/// Set the default value of the `Response` that
/// will be equivalent to a successful response.
impl Default for Response {
    fn default() -> Self {
        Self {
            status_code: 200,
            status_txt:  String::from("OK"),
            headers:     None,
            body:        vec![],
        }
    }
}

/// Converts the `Response into a `String` when
/// sending it.
impl From<Response> for Vec<u8> {
    fn from(res: Response) -> Vec<u8> {
        let mut bytes = format!(
            "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\n\r\n",
            &res.status_code,
            &res.status_txt,
            &res.headers(),
            &res.body.len(),
        )
        .as_bytes()
        .to_vec();

        bytes.extend(res.body);
        bytes
    }
}

/// Converts
impl From<HttpStatus> for Response {
    fn from(err: HttpStatus) -> Self {
        let mut ctx = Context::new();
        ctx.insert("status_code", &err.status_code);
        ctx.insert("status_text", &err.message);

        let page = TEMPLATES
            .render("error.html", &ctx)
            .unwrap_or_else(|_| {
                format!(
                    "<h1>{}</h1><p>{}</p>",
                    err.status_code, err.message
                )
            });

        Self {
            status_code: err.status_code,
            status_txt:  err.message,
            headers:     Some(HashMap::from([(
                "Content-Type".to_string(),
                "text/html".to_string(),
            )])),
            body:        page.as_bytes().to_vec(),
        }
    }
}

impl Response {
    pub fn ok(headers: Option<Headers>, body: Vec<u8>) -> Self {
        let mut response = Response::default();

        response.set_headers(headers);
        response.status_txt = "OK".to_string();
        response.body = body;

        response
    }

    pub fn err(err_page: &str, headers: Option<Headers>) -> Self {
        let ctx = Context::new();
        let http_err = HttpStatus::from(err_page.to_string());

        let page = match TEMPLATES.render(err_page, &ctx) {
            Ok(p) => p,
            Err(e) => return Self::from(HttpStatus::from(AppErr::from(e))),
        };

        let mut response = Response::default();
        response.status_code = http_err.status_code;
        response.status_txt = http_err.message;
        response.body = page.as_bytes().to_vec();
        response.set_headers(headers);

        response
    }

    pub fn set_headers(&mut self, headers: Option<Headers>) {
        self.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert(
                    "Content-Type".to_string(),
                    "text/html".to_string(),
                );
                Some(h)
            }
        };
    }

    pub fn headers(&self) -> String {
        match &self.headers {
            Some(h) => {
                let mut header_string: String = "".into();

                for (k, v) in h.iter() {
                    header_string = format!("{}{}:{}\r\n", header_string, k, v);
                }

                header_string
            }
            None => "".into(),
        }
    }
}
