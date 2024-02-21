use crate::{
    package::{find_ememnu_package_files, find_qr_package_files, get_packages_by_files, Package},
    util::{link, read_json},
    Command,
};
use anyhow::Result;
use clap::{builder::Str, Parser};
use std::{env::current_dir, path::PathBuf};

use std::fmt::Debug;

#[derive(Clone)]
pub struct MatchList {
    src: PathBuf,
    target: PathBuf,
    name: String,
}

impl Debug for MatchList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}  ---->   {}\n",
            self.src.display(),
            self.target.display()
        )
    }
}

#[derive(Parser, Clone, Debug)]
#[command(about = "link emenu-mini-core packages to qr-mini-pay ", alias = "l")]
pub struct LinkCommand {
    /// the source diretory. emenu-mini-core
    #[arg(long, short)]
    pub source: Option<PathBuf>,
    /// the target diretory. qr-mini-pay
    #[arg(long, short)]
    pub target: Option<PathBuf>,
}

impl LinkCommand {

    pub fn get_default_value(dir:Option<PathBuf>) -> PathBuf {

        if dir.is_none() {
            return current_dir().unwrap();
        }else {
            return dir.unwrap();
        }
    }
    pub fn run(command: &Command, options: &LinkCommand) -> Result<()> {
        let LinkCommand { source, target } = options.to_owned();


        let src_path = Self::get_default_value(source);
        let target_path = Self::get_default_value(target);

        if src_path == target_path {
            println!("源地址和目标地址不能相同");
            return Ok(());
        }

        let src_package_files = find_ememnu_package_files(&src_path).unwrap();
        let target_package_files = find_qr_package_files(&target_path).unwrap();

        let match_lists =
            Self::find_match_packages(&src_package_files, &target_package_files).unwrap();

        let match_node_modules_lists = Self::find_match_node_modules(&match_lists).unwrap();
        let match_miniprogram_lists = Self::find_match_miniprogram(&match_lists).unwrap();

        Self::link_match_lists(&match_node_modules_lists).unwrap();
        Self::link_match_lists(&match_miniprogram_lists).unwrap();

        Self::print_result(&match_node_modules_lists, &match_miniprogram_lists);

        Ok(())
    }

    pub fn print_result(
        node_modules_match_lists: &Vec<MatchList>,
        miniprogram_match_lists: &Vec<MatchList>,
    ) {
        if node_modules_match_lists.is_empty() && miniprogram_match_lists.is_empty() {
            println!("没找到对应的npm包进行链接");
            return;
        }

        println!("链接成功!\r\r");

        if !node_modules_match_lists.is_empty() {
            println!("node_modules目录链接列表:\r");

            for ele in node_modules_match_lists {
                print!("{:?}", ele);
            }
        }

        println!("\n\n");

        if !miniprogram_match_lists.is_empty() {
            println!("miniprogram目录链接列表:\r");
            for ele in miniprogram_match_lists {
                print!("{:?}", ele);
            }
        }
    }

    pub fn link_match_lists(match_lists: &Vec<MatchList>) -> Result<()> {
        for item in match_lists {
            let MatchList { src, target, .. } = item;
            link(src, target).unwrap();
        }

        Ok(())
    }

    /**
     * 找到miniprogram对应的目录
     */

    pub fn find_match_miniprogram(match_lists: &Vec<MatchList>) -> Result<Vec<MatchList>> {
        let mut res: Vec<MatchList> = vec![];

        for item in match_lists {
            let src = item.src.to_owned();
            let src_package: Package = read_json(&src).unwrap();
            let miniprogram = src_package.miniprogram;
            let name = src_package.name.clone();
            let src_parent = src.parent().unwrap().to_owned();

            let src_dir = src_parent.join(miniprogram);

            if !src_dir.exists() {
                continue;
            }

            let target = item.target.to_owned();

            let mut target_parent = target.parent().unwrap().to_owned();

            target_parent.push("miniprogram_npm");

            if !target_parent.exists() {
                continue;
            }

            target_parent.push(name.clone());

            res.push(MatchList {
                src: src_dir.clone(),
                target: target_parent.clone(),
                name: name.clone(),
            })
        }
        Ok(res)
    }

    /**
     *
     * 找到node_modules对应的目录
     */

    pub fn find_match_node_modules(match_lists: &Vec<MatchList>) -> Result<Vec<MatchList>> {
        let mut res: Vec<MatchList> = vec![];
        for item in match_lists {
            let target = item.target.to_owned();
            let name = item.name.to_owned();
            let mut parent = target.parent().unwrap().to_owned();
            parent.push("node_modules");
            parent.push(name);
            res.push(MatchList {
                src: item.src.parent().unwrap().to_owned(),
                target: parent,
                name: item.name.clone(),
            })
        }

        Ok(res)
    }

    pub fn find_match_packages(
        src_package_files: &Vec<PathBuf>,
        target_package_files: &Vec<PathBuf>,
    ) -> Result<Vec<MatchList>> {
        let src_packages: Vec<Package> = get_packages_by_files(src_package_files);
        let target_packages: Vec<Package> = get_packages_by_files(target_package_files);

        let mut res: Vec<MatchList> = vec![];

        for pkg in target_packages {
            pkg.iteral_all().for_each(|dep| {
                for src_pkg in src_packages.clone() {
                    if src_pkg.name == dep.name {
                        res.push(MatchList {
                            name: dep.name.clone(),
                            src: src_pkg.path.clone(),
                            target: dep.path.clone(),
                        })
                    }
                }
            })
        }

        Ok(res)
    }
}
