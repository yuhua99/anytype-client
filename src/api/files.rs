use std::{fs, path::Path};

use anyhow::Result;
use reqwest::{Method, multipart};

use super::AnytypeClient;
use crate::models::FileUploadResponse;

impl AnytypeClient {
    pub async fn upload_file(&self, space_id: &str, path: &Path) -> Result<FileUploadResponse> {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file")
            .to_string();
        let part = multipart::Part::bytes(fs::read(path)?).file_name(file_name);
        let form = multipart::Form::new().part("file", part);
        self.request_multipart(&super::space_files_path(space_id), form)
            .await
    }

    pub async fn download_file(
        &self,
        space_id: &str,
        file_id: &str,
        width: Option<i64>,
    ) -> Result<Vec<u8>> {
        let path = match width {
            Some(width) => super::space_file_path_with_width(space_id, file_id, width),
            None => super::space_file_path(space_id, file_id),
        };
        self.request_bytes(Method::GET, &path).await
    }

    pub async fn delete_file(&self, space_id: &str, file_id: &str, skip_bin: bool) -> Result<()> {
        let path = super::space_file_path_with_skip_bin(space_id, file_id, skip_bin);
        self.request_empty(Method::DELETE, &path, Option::<&()>::None)
            .await
    }
}
