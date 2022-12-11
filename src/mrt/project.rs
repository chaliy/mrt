use std::path::PathBuf;
use anyhow::{Result, Context};
use glob::glob;
use log::warn;

use crate::package::{Package, PackageStatus};
use crate::manifest::Manifest;

#[derive(Debug)]
pub struct Project {
    root_path: PathBuf,
    manifest: Manifest,
}

impl Project {

    /// Read project from manifest path if defined. if not will use current directory and mrt.yml file
    pub fn read(manifest_path: Option<PathBuf>) -> Result<Project> {

        let root_path = match manifest_path {
            Some(path) => {
                // Here means user specified manifest path 
                // and we assume he knows what he is doing
                let manifest_path = path.canonicalize()?;

                manifest_path.parent()
                    .with_context(|| format!("Manifest path {:?} has no parent, cannot determine project root", manifest_path))?
                    .canonicalize()?
            },
            None => std::env::current_dir()?
        };

        Ok(Project {
            root_path,
            manifest: Manifest::new()
        })
    }
    
    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_path
    }

    pub fn read_package(&self, package_path: PathBuf) -> Result<Package> {
        return Package::from_package_path(
            self.root_path.join(package_path),
            self.root_path.clone()
        );
    }

    pub fn get_packages(&self, all: bool) -> Vec<Package> {

        let mut packages: Vec<Package> = vec![];

        for package_glob in &self.manifest.packages {
            let rooted_package_glob = self.root_path.join(&package_glob);

            let full_glob = rooted_package_glob.to_str()
                .expect(&format!("Convert path `{}` to string", rooted_package_glob.display()));

            match glob(&full_glob) {
                Ok(paths) => {
                    paths
                        .filter_map(Result::ok)
                        .filter(|path| path.is_dir())
                        .for_each(|path| {
                            let package = self.read_package(path).unwrap();

                            match package.status {
                                PackageStatus::Valid => {
                                    packages.push(package);
                                },
                                PackageStatus::CannotRead(ref message) => {
                                    if all {
                                        packages.push(package);
                                    } else {
                                        warn!("Cannot read package at path '{}': {}", package.path, message);
                                    }
                                },
                                PackageStatus::CannotDetectArchetype => {
                                    if all {
                                        packages.push(package);
                                    } else {
                                        warn!("Cannot detect package archetype at path '{}'", package.path);
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