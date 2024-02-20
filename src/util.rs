
use std::{fs::{self, read_to_string}, path::PathBuf};
use serde::de::DeserializeOwned;
use anyhow::Result;

pub fn change_relative_to_abs(path:&PathBuf) -> PathBuf {
    fs::canonicalize(path).expect(&format!("请检查路径{:?}",path))
}



pub  fn read_json<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let str = path.to_str().unwrap();
    let content = read_to_string(str).expect(&format!("read file error {:?}",path));
    Ok(serde_json::from_str(&content)?)
}
