use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

use crate::archetypes::detect_archetype;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub path: String,
    pub absolute_path: PathBuf,
    pub archetype_id: String,
    pub status: PackageStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PackageStatus {
    Valid,
    CannotRead(String),
    CannotDetectArchetype,
}

impl Display for PackageStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageStatus::Valid => write!(f, "Valid"),
            PackageStatus::CannotRead(message) => write!(f, "Error: {}", message),
            PackageStatus::CannotDetectArchetype => write!(f, "Cannot detect archetype"),
        }
    }
}

impl Package {
    pub fn from_package_path(package_path: PathBuf, project_path: PathBuf) -> Result<Package> {
        let absolute_path = package_path.canonicalize()?;

        let path = absolute_path
            .strip_prefix(&project_path)?
            .to_str()
            .unwrap_or("n/a")
            .to_string();

        match detect_archetype(&absolute_path) {
            Some(archetype) => {
                match archetype.get_info_extractor(&absolute_path) {
                    Ok(extractor) => Ok(Package {
                        name: extractor.get_name().to_string(),
                        version: extractor.get_version().to_string(),
                        path,
                        absolute_path,
                        archetype_id: archetype.get_id().to_string(),
                        status: PackageStatus::Valid,
                    }),
                    Err(err) => {
                        // Archetype detected, but cannot read package info
                        Ok(Package {
                            name: String::from("n/a"),
                            version: String::from("n/a"),
                            path,
                            absolute_path,
                            archetype_id: archetype.get_id().to_string(),
                            status: PackageStatus::CannotRead(err.to_string()),
                        })
                    }
                }
            }
            None => Ok(Package {
                name: String::from("n/a"),
                version: String::from("n/a"),
                path,
                absolute_path,
                archetype_id: String::default(),
                status: PackageStatus::CannotDetectArchetype,
            }),
        }
    }
}

pub trait PackageInfoExtractor {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> &str;
}

pub struct NoopPackageInfoExtractor {}

impl PackageInfoExtractor for NoopPackageInfoExtractor {
    fn get_name(&self) -> &str {
        "noop"
    }

    fn get_version(&self) -> &str {
        "noop"
    }
}
