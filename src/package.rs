use anyhow::Result;
use clap::Parser;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

use crate::util::{change_relative_to_abs, read_json};

pub fn glob_find_package_files(src_abs: PathBuf) -> Result<Vec<PathBuf>> {
    let entrys = glob(src_abs.to_str().unwrap()).expect("Failed to read glob pattern");

    let mut res = vec![];

    entrys.for_each(|entry| {
        let entry = entry.unwrap();
        res.push(entry);
    });

    Ok(res)
}

pub fn find_ememnu_package_files(src: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut src_abs = change_relative_to_abs(src);

    src_abs.push("packages/*/package.json");

    glob_find_package_files(src_abs)
}

pub fn find_qr_package_files(src: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut src_abs = change_relative_to_abs(src);
    src_abs.push("*/package.json");
    glob_find_package_files(src_abs)
}

pub fn get_packages_by_files(dirs: &Vec<PathBuf>) -> Vec<Package> {
    dirs.into_iter()
        .map(|p| {
            let mut pkg = read_json::<Package>(p).unwrap();
            pkg.path = p.to_owned();
            pkg
        })
        .collect()
}



#[derive(Parser, Clone, Debug)]
pub struct ListTarget {
    pub name:String,
    pub path:PathBuf
}

pub fn get_npm_path(packages:&Vec<Package>,dir:&str)->Result<Vec<ListTarget>> {

    let mut res = vec![];
    for ele in packages {
        let npms :Vec<DepReq>= ele.iteral_all().map(|p| {
            return p.clone();
        }).collect();
        for ele in npms {
            let name = &ele.name;
            let mut path = ele.path.clone().parent().unwrap().to_path_buf();
            path.push(dir);
            path.push(name);
            res.push(ListTarget {
                name:name.clone(),
                path
            })
        }
        
    }

    Ok(res)
}


#[derive(Clone, Debug)]
pub struct DepReq {
    pub name: String,
    /// package.json的地址
    pub path: PathBuf,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Package {
    pub name: String,
    pub dependencies: BTreeMap<String, String>,
    pub dev_dependencies: BTreeMap<String, String>,
    pub miniprogram: String,
    /// package.json的文件地址
    pub path: PathBuf,
}

impl Package {
    pub fn iteral_all(&self) -> impl Iterator<Item = DepReq> + '_ {
        self.dependencies
            .iter()
            .chain(self.dev_dependencies.iter())
            .map(|(name, _)| DepReq {
                name: name.to_owned(),
                path: self.path.clone(),
            })
    }
}
