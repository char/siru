use std::path::{Path, PathBuf};

use tokio::{fs::ReadDir, io::Result};

use crate::logging::log_addition;

#[derive(Clone)]
pub struct SiruIO {
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl SiruIO {
    pub fn get_source_path(&self, unprefixed_path: impl AsRef<Path>) -> impl AsRef<Path> {
        let path = unprefixed_path.as_ref();
        if path.is_absolute() {
            path.to_owned()
        } else {
            self.source_dir.join(unprefixed_path).as_path().to_owned()
        }
    }

    pub fn get_output_path(&self, unprefixed_path: impl AsRef<Path>) -> impl AsRef<Path> {
        let path = unprefixed_path.as_ref();
        if path.is_absolute() {
            path.to_owned()
        } else {
            self.output_dir.join(unprefixed_path).as_path().to_owned()
        }
    }

    pub async fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
        let path = self.get_source_path(path);
        tokio::fs::read(path).await
    }

    pub async fn read_to_string(&self, path: impl AsRef<Path>) -> Result<String> {
        let path = self.get_source_path(path);
        tokio::fs::read_to_string(path).await
    }

    pub async fn read_dir(&self, path: impl AsRef<Path>) -> Result<(ReadDir, PathBuf)> {
        let path = self.get_source_path(path);
        tokio::fs::read_dir(&path)
            .await
            .map(|dir| (dir, path.as_ref().into()))
    }

    pub async fn write(
        &self,
        unprefixed_path: impl AsRef<Path>,
        content: impl AsRef<[u8]>,
    ) -> Result<()> {
        let path = self.get_output_path(&unprefixed_path);
        log_addition(unprefixed_path.as_ref().as_os_str().to_string_lossy());
        tokio::fs::write(path, content).await
    }

    pub async fn copy(
        &self,
        source_path: impl AsRef<Path>,
        dest_path: impl AsRef<Path>,
    ) -> Result<u64> {
        let source_path = self.get_source_path(&source_path);
        let dest_path = self.get_output_path(&dest_path);
        log_addition(dest_path.as_ref().as_os_str().to_string_lossy());
        tokio::fs::copy(source_path, dest_path).await
    }

    pub async fn create_dir(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = self.get_output_path(&path);
        tokio::fs::create_dir(path).await
    }

    pub async fn create_dir_all(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = self.get_output_path(&path);
        tokio::fs::create_dir_all(path).await
    }
}
