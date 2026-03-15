use {
    super::{
        external::Http,
        file_system::FileSystem,
    },
    crate::{
        debug,
        message::{
            Headers,
            Request,
            Resource,
            Response,
        },
        server::{
            external::Cgi,
            Server,
        },
        utils::{
            AppErr,
            HttpResult,
            HttpStatus,
            HTTP,
            TEMPLATES,
        },
    },
    std::{
        fs,
        io::Write,
        path::Path,
    },
    tera::Context,
};

impl Http {
    pub fn handle(req: &Request, server: &Server) -> HttpResult<Response> {
        // Get the path of static page resource being requested
        let Resource::Path(s) = &req.resource;

        let route: Vec<&str> = s.split("/").collect();

        let mut path = format!("public{}", s);
        if route.len() > 1 && route[1] == "public" {
            path = route[1..].join("/");
        }

        if let Some(default_file) = server.find_default_file(s) {
            return Self::serve_default(&default_file, s, server.listing(), server);
        }

        match !s.contains(".") {
            true => Self::serve_default(
                "index.html",
                debug!(&path),
                server.listing(),
                server,
            ),
            _ => Self::serve_static(&path, server.listing()),
        }
    }

    fn serve_default(tmpl: &str, path: &str, list: bool, server: &Server) -> HttpResult<Response> {
        let mut ctx = Context::new();
        ctx.insert("title", "Rust");
        if server
            .find_default_file(path)
            .is_none()
            || path == "/"
        {
            let format_path = if path == "/" { "public/" } else { path };
            match FileSystem::listing(debug!(format_path), list) {
                Ok(items) => {
                    ctx.insert("list", debug!(&items));
                    ctx.insert("Size", &items.len());
                }
                Err(e) => {
                    return Err(HttpStatus::from(AppErr::from(e)));
                }
            }
        }

        dbg!(&path);

        let page = TEMPLATES
            .render(&tmpl, &ctx)
            .map_err(|e| AppErr::from(debug!(e)));

        match page {
            Ok(page) => Ok(Response::ok(None, page.as_bytes().to_vec())),
            Err(e) => Err(HttpStatus::from(AppErr::from(e))),
        }
    }

    fn serve_static(path: &str, list: bool) -> HttpResult<Response> {
        if !list {
            return Err(HttpStatus::from(403));
        }

        let mut headers = Headers::new();

        let mut cgi = false;
        // Déterminer le Content-Type en fonction de l'extension du fichier
        let content_type = match path.split('.').last() {
            Some("css") => "text/css",
            Some("js") => "text/javascript",
            Some("html") => "text/html",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("py") => {
                cgi = true;
                "text/plain"
            }
            Some("svg") => "image/svg+xml",
            Some("txt") => "text/plain",
            Some("pdf") => "application/pdf",
            Some("xml") => "application/xml",
            Some("zip") => "application/zip",
            _ => "text/plain",
        };

        headers.insert(
            "Content-Type".to_string(),
            content_type.to_string(),
        );

        println!("final path{}", path);

        if !Path::exists(Path::new(&path)) {
            return Err(HttpStatus::from(404));
        }
        // Charger le contenu du fichier
        let content = fs::read(&path)?;

        if cgi {
            match Cgi::interprete_python(&path) {
                Ok((cgi_headers, res)) => Ok(Response::ok(Some(cgi_headers), res)),
                Err(err) => {
                    eprintln!("Erreur d'exécution Python: {:?}", err);
                    return Err(debug!(HttpStatus::from(500)));
                }
            }
        }
        else {
            Ok(Response::ok(Some(headers), content))
        }
    }

    pub fn serve_auth(
        route: &str,
        fichier: &str,
        server: &Server,
        stream: &mut impl Write,
        addr: &str,
    ) -> HttpResult<Response> {
        if server
            .find_default_file(route)
            .is_none()
        {
            return Err(debug!(HttpStatus::from(404)));
        }
        let mut ctx = Context::new();
        ctx.insert("title", "Rust");

        let _page = TEMPLATES
            .render(&fichier, &ctx)
            .map_err(|e| AppErr::from(debug!(e)))?;

        let addr = format!(
            "http://{}:{}{}",
            server.host(),
            addr.split(":")
                .last()
                .unwrap(),
            route
        );
        dbg!(addr.clone());
        let response = format!(
            "HTTP/1.1 301 Found\r\nLocation: {}\r\n\r\n",
            addr
        );
        stream.write_all(response.as_bytes())?;
        stream.flush()?;

        Ok(Response::ok(
            Some(Headers::new()),
            Vec::new(),
        ))
    }

    pub fn set_cookie() -> HttpResult<Response> {
        let session_id = match HTTP.write() {
            Ok(mut http) => http
                .session_store
                .create_session(),
            Err(e) => {
                debug!(e);
                return Err(debug!(HttpStatus::from(500)));
            }
        };
        let mut headers = Headers::new();
        headers.insert(
            "Set-Cookie".to_string(),
            format!("session_id={}; Path=/; HttpOnly", session_id),
        );
        headers.insert("Location".to_string(), "/".to_string());

        let mut response = Response::from(HttpStatus::from(301));
        response.set_headers(Some(headers));

        Ok(response)
    }
}
