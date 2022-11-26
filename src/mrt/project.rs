use std::path::PathBuf;
use anyhow::Result;
use glob::glob;
use log::warn;

use crate::package::{Package, PackageStatus};
use crate::manifest::Manifest;

pub struct Project {
    _manifest_path: PathBuf,
    root_path: PathBuf,
    manifest: Manifest,
}

impl Project {
    pub fn new(manifest_path: PathBuf) -> Project {
        let root_path = match manifest_path.parent() {
            Some(parent) => parent.to_path_buf(),
            None => std::env::current_dir().unwrap_or_default()
        };

        Project {
            _manifest_path: manifest_path,
            root_path,
            manifest: Manifest::new()
        }
    }

    pub fn get_packages(&self, all: bool) -> Vec<Package> {

        let mut packages: Vec<Package> = vec![];

        for package_glob in &self.manifest.packages {
            let rooted_package_glob = self.root_path.join(&package_glob);

            let full_glob = rooted_package_glob.to_str()
                .expect(&format!("convert path `{}` to string", rooted_package_glob.display()));


            match glob(&full_glob) {
                Ok(paths) => {
                    paths
                        .filter_map(Result::ok)
                        .filter(|path| path.is_dir())
                        .for_each(|path| {
                            let package = Package::from_package_path(path);

                            match package.status {
                                PackageStatus::Valid => {
                                    packages.push(package);
                                },
                                PackageStatus::CannotRead => {
                                    if all {
                                        packages.push(package);
                                    } else {
                                        warn!("Cannot read package at path '{}': {}", package.path.display(), package.status_message.unwrap());
                                    }
                                },
                                PackageStatus::CannotDetect => {
                                    if all {
                                        packages.push(package);
                                    } else {
                                        warn!("Cannot detect package archetype at path '{}': {}", package.path.display(), package.status_message.unwrap());
                                    }
                                }
                            }
                        });
                },
                Err(err) => {
                    warn!("Package glob {} failed: {}. Ignoring.", &full_glob, err);
                    continue;
                }
            }
        }

        packages
    }
}