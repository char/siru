use std::{
    fs::{self, ReadDir},
    io::Result,
    path::{Path, PathBuf},
};

fn cat_relative_path(source_dir: impl AsRef<Path>, path: impl AsRef<Path>) -> impl AsRef<Path> {
    let path = path.as_ref();
    if path.is_absolute() {
        path.to_owned()
    } else {
        source_dir.as_ref().join(path).as_path().to_owned()
    }
}

pub trait SiruFS {
    fn get_source_dir(&self) -> &PathBuf;
    fn get_output_dir(&self) -> &PathBuf;

    fn read(&self, path: impl AsRef<Path>) -> Result<String> {
        fs::read_to_string(cat_relative_path(self.get_source_dir(), path))
    }

    fn read_bin(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
        fs::read(cat_relative_path(self.get_source_dir(), path))
    }

    fn resolve(&self, dir: impl AsRef<Path>, suffix: impl AsRef<Path>) -> Result<PathBuf> {
        cat_relative_path(cat_relative_path(self.get_source_dir(), dir), suffix)
            .as_ref()
            .canonicalize()
    }

    fn read_dir(&self, path: impl AsRef<Path>) -> Result<ReadDir> {
        fs::read_dir(cat_relative_path(self.get_source_dir(), path))
    }

    fn get_write_pipeline(&self) -> &WritePipeline;

    fn write(&self, path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
        let path = cat_relative_path(self.get_output_dir(), path)
            .as_ref()
            .to_path_buf();
        let contents = contents.as_ref().to_vec();

        let mut pipeline = self.get_write_pipeline().clone();
        pipeline.push(|path, contents| {
            fs::write(path, contents)?;
            Ok(vec![])
        });

        self.iter_write_pipeline(path, contents, pipeline.as_slice())
    }

    fn iter_write_pipeline(
        &self,
        path: PathBuf,
        contents: Vec<u8>,
        pipeline: &[WriteHook],
    ) -> Result<()> {
        let (next, rest) = pipeline.split_at(1);
        let next = next[0];

        let results = next(path, contents)?;

        if !rest.is_empty() {
            for (path, contents) in results {
                self.iter_write_pipeline(path, contents, rest)?;
            }
        }

        Ok(())
    }
}

pub type WritePipeline = Vec<WriteHook>;
pub type WriteHook = fn(path: PathBuf, contents: Vec<u8>) -> Result<Vec<(PathBuf, Vec<u8>)>>;
