use {
    super::external::Upload,
    crate::{
        debug,
        message::{
            Request,
            Resource,
            Response,
        },
        utils::{
            find_bytes,
            AppErr,
            HttpResult,
            HttpStatus,
            BOUNDARY_REGEX,
            CONTENT_DISPOSITION_REGEX,
            TEMPLATES,
        },
    },
    std::{
        collections::HashMap,
        env,
        fs::{
            self,
            File,
        },
        io::Write,
        path::Path,
    },
    tera::Context,
};

impl Upload {
    pub fn handle(req: &Request) -> HttpResult<Response> {
        let Resource::Path(_) = &req.resource;

        // Vérification améliorée du type de contenu multipart/form-data
        match req
            .headers
            .get("Content-Type")
        {
            Some(content_type)
                if content_type
                    .as_bytes()
                    .windows(b"multipart/form-data".len())
                    .any(|w| w == b"multipart/form-data") =>
            {
                Self::process_multipart_upload(req)
            }
            _ => Err(debug!(HttpStatus::from(400))), // Erreur si ce n'est pas du multipart
        }
    }

    fn process_multipart_upload(req: &Request) -> HttpResult<Response> {
        if req.body.is_empty() {
            println!("ERROR: Upload body is empty!");
            return Err(HttpStatus::from(400)); // Bad Request
        }

        let content_type = req
            .headers
            .get("Content-Type")
            .ok_or_else(|| {
                println!("No Content-Type header found");
                HttpStatus::from(400)
            })?;

        let boundary = match BOUNDARY_REGEX.captures(content_type) {
            Some(caps) => {
                let boundary_value = caps[1].to_string();
                format!("--{}", boundary_value)
            }
            None => {
                println!(
                    "Could not extract boundary from: {}",
                    content_type
                );
                return Err(HttpStatus::from(400));
            }
        };

        let default_dir = format!(
            "{}/public/uploads",
            env!("CARGO_MANIFEST_DIR")
        );

        let upload_dir = env::var("UPLOAD_DIR").unwrap_or(default_dir);

        if !Path::new(&upload_dir).exists() {
            fs::create_dir_all(&upload_dir).map_err(|e| {
                println!("Failed to create upload directory: {:?}", e);
                AppErr::from(e)
            })?;
        }

        let boundary_bytes = boundary.as_bytes();
        let parts = split_by_boundary(&req.body, boundary_bytes);

        let mut upload_results = Vec::new();
        let separator = b"\r\n\r\n";

        for (_, part) in parts
            .iter()
            .enumerate()
            .skip(1)
        {
            if part.starts_with(b"--") || part.is_empty() {
                continue;
            }
            // println!("<=====part : {:?}",String::from_utf8_lossy(&part));
            let part_str = String::from_utf8_lossy(&part);

            match CONTENT_DISPOSITION_REGEX.captures(&part_str.to_string()) {
                Some(caps) => {
                    let field_name = extract_filename(&req.body).unwrap();
                    if let Some(_) = caps.get(3) {
                        let filename = field_name
                            .as_str()
                            .to_string();

                        if let Some(index) = part
                            .windows(separator.len())
                            .position(|w| w == separator)
                        {
                            let content_start = index + separator.len();
                            let content = &part[content_start..];

                            let file_path = format!("{}/{}", upload_dir, filename);

                            match File::create(&file_path) {
                                Ok(mut file) => match file.write_all(content) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        println!("File write error: {:?}", e);
                                    }
                                },
                                Err(e) => {
                                    println!("File creation error: {:?}", e);
                                }
                            }
                        }
                    }
                    else {
                        if let Some(index) = part
                            .windows(separator.len())
                            .position(|w| w == separator)
                        {
                            let content_start = index + separator.len();
                            let content = &part[content_start..];

                            let content_str = match std::str::from_utf8(content) {
                                Ok(str) => str.trim_end_matches("\r\n"),
                                Err(_) => {
                                    println!("Invalid UTF-8 in form field, skipping.");
                                    continue;
                                }
                            };

                            upload_results.push(format!(
                                "Form field: {} = {}",
                                field_name, content_str
                            ));
                        }
                    }
                }
                None => {
                    println!("No content disposition found in part");
                }
            }
        }

        let mut ctx = Context::new();
        ctx.insert("title", "Upload Complete");
        ctx.insert("results", &upload_results);

        let page = TEMPLATES
            .render("upload_success.html", &ctx)
            .unwrap_or_else(|_| {
                let results_html = upload_results.join("<br>");
                format!(
                    "<html><head><title>Upload Complete</title></head>
                <body><h1>Upload Complete</h1><p>{}</p>
                <a href='/'>Back to home</a></body></html>",
                    results_html
                )
            });

        let mut headers = HashMap::new();
        headers.insert(
            "Content-Type".to_string(),
            "text/html".to_string(),
        );

        Ok(Response::ok(
            Some(headers),
            page.as_bytes().to_vec(),
        ))
    }
}

fn split_by_boundary<'a>(body: &'a [u8], boundary: &'a [u8]) -> Vec<&'a [u8]> {
    let mut parts = Vec::new();
    let mut start = 0;

    for (i, window) in body
        .windows(boundary.len())
        .enumerate()
    {
        if window == boundary {
            parts.push(&body[start..i]); // Ajouter la partie avant le boundary
            start = i + boundary.len(); // Déplacer le début après le
        }
    }

    // Ajouter la dernière partie après le dernier boundary
    if start < body.len() {
        parts.push(&body[start..]);
    }

    parts
}

fn extract_filename(part: &[u8]) -> Option<String> {
    let need = b"filename=";
    find_bytes(part, need).map(|filename_start| {
        let filename_end =
            find_bytes(&part[filename_start..], b"\r\n").map_or(part.len(), |p| filename_start + p);

        // Extraire la chaîne brute
        let raw_filename = &part[filename_start + 9..filename_end];

        // Supprimer les guillemets si présents
        let raw_filename = if raw_filename.len() >= 2
            && raw_filename[0] == b'"'
            && raw_filename[raw_filename.len() - 1] == b'"'
        {
            &raw_filename[1..raw_filename.len() - 1]
        }
        else {
            raw_filename
        };

        // Essayer plusieurs décodages
        match String::from_utf8(raw_filename.to_vec()) {
            Ok(utf8_name) => urlencoding::decode(&utf8_name)
                .unwrap()
                .into_owned(),
            Err(_) => {
                // Fallback à l'encodage ISO-8859-1 (Latin-1) souvent utilisé pour les
                // caractères accentués
                let latin1_name = raw_filename
                    .iter()
                    .map(|&c| c as char)
                    .collect::<String>();
                urlencoding::decode(&latin1_name)
                    .unwrap()
                    .into_owned()
            }
        }
    })
}
