use crate::{
    package::{find_ememnu_package_files, find_qr_package_files, get_packages_by_files, Package},
    Command,
};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;


#[derive(Clone, Debug)]
pub struct MatchPackage {
    src: PathBuf,
    target: PathBuf,
}

#[derive(Parser, Clone, Debug)]
#[command(about = "link emenu-mini-core packages to qr-mini-pay ", alias = "l")]
pub struct LinkCommand {
    /// the source diretory. emenu-mini-core
    #[arg(long, short)]
    pub source: String,
    /// the target diretory. qr-mini-pay
    #[arg(long, short)]
    pub target: String,
}

impl LinkCommand {
    pub fn run(command: &Command, options: &LinkCommand) -> Result<()> {
        let LinkCommand { source, target } = options.to_owned();

        let src_path = PathBuf::from(source);
        let target_path = PathBuf::from(target);

        let src_package_files = find_ememnu_package_files(&src_path).unwrap();
        let target_package_files = find_qr_package_files(&target_path).unwrap();

        let match_lists = Self::find_match_packages(&src_package_files, &target_package_files);

        println!("match_lists  is {:?}", match_lists);

        Ok(())
    }

    pub fn find_match_packages(
        src_package_files: &Vec<PathBuf>,
        target_package_files: &Vec<PathBuf>,
    ) -> Result<Vec<MatchPackage>> {
        let src_packages: Vec<Package> = get_packages_by_files(src_package_files);
        let target_packages: Vec<Package> =get_packages_by_files(target_package_files);

        let mut res:Vec<MatchPackage> = vec![];

        for pkg in target_packages {
            pkg.iteral_all().for_each(|dep|  {
                for src_pkg in src_packages.clone() {
                    if src_pkg.name == dep.name {
                        res.push(MatchPackage {
                            src : src_pkg.path.clone(),
                            target:dep.path.clone()
                        })
                    }
                }
            })
        }

       Ok(res)
    }
}
