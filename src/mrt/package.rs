use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::archetypes::{match_archetype, get_archetype_by_id};
use crate::runners::{PackageScriptRunContext, PackageScriptRunResult};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Package {
    pub name: String,
    pub path: PathBuf,
    pub archetype_id: String,
    pub status: PackageStatus,
    pub status_message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PackageStatus {
    Valid,
    CannotRead,
    CannotDetect
}

impl Package {
    fn unknown(package_path: PathBuf) -> Package {
        Package {
            name: String::from("unknown"),
            path: package_path,
            archetype_id: String::from("unknown"),
            status: PackageStatus::CannotDetect,
            status_message: Some(String::from("Cannot detect package archetype"))
        }
    }
    fn error(package_path: PathBuf, err: anyhow::Error) -> Package {
        Package {
            name: String::from("error"),
            path: package_path,
            archetype_id: String::from("error"),
            status: PackageStatus::CannotRead,
            status_message: Some(err.to_string())
        }
    }
    pub fn from_package_path(package_path: PathBuf) -> Package {
        match match_archetype(&package_path) {
            Some(archetype) => {
                match archetype.get_info_extractor(&package_path) {
                    Ok(extractor) => {
                        return Package {
                            name: extractor.get_name().to_string(),
                            path: package_path,
                            archetype_id: archetype.get_id().to_string(),
                            status: PackageStatus::Valid,
                            status_message: None
                        }
                    },
                    Err(err) => {
                        return Package::error(package_path, err);
                    },
                }
            },
            None => {
                return Package::unknown(package_path);
            }
        }
    }
    
    pub fn run_script(&self, script_spec: &String) -> PackageScriptRunResult {

        println!("Running script '{script_spec}' for package '{}'", self.name);

        let script_runner = get_archetype_by_id(self.archetype_id.as_str())
            .unwrap()
            .get_script_runner();
            
        return script_runner.run_script(script_spec, &PackageScriptRunContext {
            package: self
        });
    }

    
}

pub trait PackageInfoExtractor {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> &str;
}

pub struct NoopPackageInfoExtractor {}

impl PackageInfoExtractor for NoopPackageInfoExtractor {
    fn get_name(&self) -> &str {
        return "noop";
    }

    fn get_version(&self) -> &str {
        return "noop";
    }
}
