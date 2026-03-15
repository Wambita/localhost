use {
    super::Server,
    crate::{
        debug,
        message::{
            Request,
            Resource,
            Response,
        },
        utils::{
            HttpResult,
            HttpStatus,
        },
    },
    std::fs,
};

impl Server {
    pub(super) fn delete(&self, req: &Request) -> HttpResult<Response> {
        let Resource::Path(s) = &req.resource;

        // Extraire le nom du fichier du chemin
        let parts: Vec<&str> = s.split("/").collect();
        let encoded_filename = parts
            .last()
            .unwrap_or(&"")
            .to_string();

        // Décoder le nom du fichier (gère les accents, espaces, etc.)
        let filename = match urlencoding::decode(&encoded_filename) {
            Ok(decoded) => decoded.into_owned(),
            Err(_) => return Err(debug!(HttpStatus::from(400))),
        };

        println!("delete file: {}", filename);

        let default_dir = format!(
            "{}/public/uploads",
            env!("CARGO_MANIFEST_DIR")
        );

        // Construire le chemin complet du fichier
        let path = format!("{}/{}", debug!(&default_dir), filename);

        // Vérifier que le chemin n'essaie pas de sortir du répertoire (sécurité)
        let path_obj = std::path::Path::new(&path);
        if !path_obj.starts_with(&default_dir) {
            return Err(HttpStatus::from(403)); // Forbidden
        }
        match fs::remove_file(path) {
            Ok(_) => Err(HttpStatus::from(200)),
            _ => Err(HttpStatus::from(500)),
        }
    }
}
