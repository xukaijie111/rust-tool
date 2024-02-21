use anyhow::Result;
use serde::de::DeserializeOwned;
use std::{
    fs::{self, read_link, read_to_string, remove_file},
    os::unix::fs::symlink,
    path::PathBuf,
};

pub fn change_relative_to_abs(path: &PathBuf) -> PathBuf {
    fs::canonicalize(path).expect(&format!("请检查路径{:?}", path))
}

pub fn read_json<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let str = path.to_str().unwrap();
    let content = read_to_string(str).expect(&format!("read file error {:?}", path));
    Ok(serde_json::from_str(&content)?)
}

pub fn link(src: &PathBuf, target: &PathBuf) -> Result<()> {

    match (target.exists(), target.is_dir()) {
        (true, true) => std::fs::remove_dir_all(&target)?,
        (true, false) => std::fs::remove_file(&target)?,
        (true, _) => return Err(anyhow::Error::msg("file ")),
        _ => {}
    };

    Ok(symlink(src, target)?)
}
