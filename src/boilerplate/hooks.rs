use crate::prelude::*;
use std::path::PathBuf;

pub fn mkdirs_hook(path: PathBuf, contents: Vec<u8>) -> std::io::Result<Vec<(PathBuf, Vec<u8>)>> {
    let _ = path.parent().map(std::fs::create_dir_all);
    Ok(vec![(path, contents)])
}

pub fn logging_hook(path: PathBuf, contents: Vec<u8>) -> std::io::Result<Vec<(PathBuf, Vec<u8>)>> {
    // Split and re-join by components since Windows will use a weird mix of slashes
    let components: Vec<_> = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect();

    log_addition(components.join("/"));

    Ok(vec![(path, contents)])
}
