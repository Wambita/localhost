use {
    crate::{
        debug,
        utils::AppResult,
    },
    serde::Serialize,
    std::{
        fs::{
            self,
            ReadDir,
        },
        io,
        path::Path,
        time::SystemTime,
    },
};
#[derive(Debug, Serialize)]
pub(super) struct FileSystem {
    items: Vec<Item>,
}

#[derive(Debug, Serialize)]
pub(super) struct Item {
    name:          String,
    link:          String,
    size:          u64,
    modified_at:   i64,
    is_dir:        bool,
    in_upload_dir: bool,
}

impl FileSystem {
    fn get_content<P: AsRef<Path>>(path: P) -> Result<ReadDir, io::Error> { fs::read_dir(path) }

    pub(super) fn listing<P: AsRef<Path>>(path: P, list: bool) -> AppResult<Vec<Item>> {
        let mut items = Vec::new();
        let content = Self::get_content(&path)?;
        for entry in content {
            let entry = entry?;
            let mut full_path = entry.path();
            let is_in_upload = full_path
                .parent()
                .and_then(|p| p.file_name())
                .map(|name| name == "uploads")
                .unwrap_or(false);
            let name = entry
                .file_name()
                .into_string()
                .unwrap();

            if let Ok(stripped_path) = full_path.strip_prefix("public") {
                full_path = stripped_path.to_path_buf();
            }

            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified_at = metadata.modified()?;
            let is_dir = metadata.is_dir();
            let item = Item {
                name,
                size,
                link: debug!(full_path)
                    .to_string_lossy()
                    .into_owned(),
                modified_at: modified_at
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                is_dir,
                in_upload_dir: is_in_upload,
            };

            items.push(item);
        }
        if !list {
            Ok(Vec::new())
        }
        else {
            Ok(items)
        }
    }
}
