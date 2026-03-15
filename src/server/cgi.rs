use {
    super::external::Cgi,
    crate::utils::{
        AppErr,
        AppResult,
    },
    std::{
        collections::HashMap,
        env,
        io::{
            self,
        },
        path::Path,
        process::Command,
    },
};

impl Cgi {
    pub fn interprete_python(path: &str) -> AppResult<(HashMap<String, String>, Vec<u8>)> {
        println!("Chemin reçu: {}", path);

        // Construire le chemin complet
        let full_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path);

        println!("Chemin complet: {}", full_path);

        let script_path = Path::new(&full_path);

        // Vérifier si le fichier existe
        if !script_path.exists() {
            println!("Le fichier n'existe pas: {}", full_path);
            return Err(AppErr::NotFound(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Fichier non trouvé: {}", full_path),
            )));
        }

        // Exécuter le script Python directement sans fork/exec
        let output = Command::new("python3")
            .arg(&full_path)
            .current_dir(
                script_path
                    .parent()
                    .unwrap_or(Path::new("/")),
            )
            .output()
            .map_err(|e| {
                AppErr::NotFound(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Échec de l'exécution: {}", e),
                ))
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            println!("Erreur Python: {}", error_msg);
            return Err(AppErr::NotFound(io::Error::new(
                io::ErrorKind::Other,
                format!("Le script a échoué: {}", error_msg),
            )));
        }
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut headers = HashMap::new();
        let mut body = Vec::new();
        let mut in_body = false;

        for line in output_str.lines() {
            if in_body {
                body.extend_from_slice(line.as_bytes());
                body.extend_from_slice(b"\n");
            }
            else if line.is_empty() {
                in_body = true;
            }
            else if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }
        Ok((headers, body))
    }
}
