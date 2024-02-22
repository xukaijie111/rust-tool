use std::{fs::read_link, path::PathBuf};

use anyhow::Result;
use clap::Parser;

use crate::{
    package::{
        find_qr_package_files, get_npm_path, get_packages_by_files, ListTarget
    },
    Command,
};

/**
 *
 * 列出链接的信息
 */

#[derive(Parser, Clone, Debug)]
#[command(about = "list qr_mini_pay link list ", alias = "s")]
pub struct ListCommand {
    /// the source diretory. qr_mini_pay
    #[arg(long, short)]
    pub path: PathBuf,
}

impl ListCommand {
    pub fn run(command: &Command, options: &ListCommand) -> Result<()> {
        let path = options.path.to_owned();
        let target_package_files = find_qr_package_files(&path).unwrap();

        let packages = get_packages_by_files(&target_package_files);

        let node_modules_paths = get_npm_path(&packages, "node_modules").unwrap();

        let miniprogram_paths = get_npm_path(&packages, "miniprogram_npm").unwrap();

        let lists = [node_modules_paths, miniprogram_paths].concat();

        Self::print(&lists).unwrap();

        Ok(())
    }

    pub fn print(lists: &Vec<ListTarget>) -> Result<()> {
        let mut has_link = false;
        for ele in lists {
            let path = ele.path.to_owned();
            let res = read_link(&path);
            if res.is_ok() {
                print!("{:?}  <----  {:?}\n", path, res.unwrap());
                has_link = true;
            }
        }

        if !has_link {
            println!("没有link的结果");
        }

        Ok(())
    }
}
