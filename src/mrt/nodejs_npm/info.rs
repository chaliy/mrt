use std::{path::PathBuf, fs::File, io::BufReader};
use anyhow::{Result, Context};
use serde::Deserialize;

use crate::{package::PackageInfoExtractor};

pub struct NpmPackageInfoExtractor {
    npm_package: NpmPackage
}

// Structure represents package.json info
#[derive(Deserialize, Debug)]
struct NpmPackage {
    name: String,
    version: String,
}

impl NpmPackageInfoExtractor {
    pub fn from_package_path(package_path: &PathBuf) -> Result<NpmPackageInfoExtractor> {

        let package_json_path = package_path.join("package.json");

        let package_json_file = File::open(&package_json_path)
            .with_context(|| format!("Failed to open package.json file at {}", package_json_path.display()))?;
        let package_json_reader = BufReader::new(package_json_file);

        let package_json = serde_json::from_reader(package_json_reader)
            .with_context(|| format!("Failed to parse JSON of package.json file at {}", package_json_path.display()))?;

        return Ok(NpmPackageInfoExtractor {
            npm_package: package_json
        });
    }
}

impl PackageInfoExtractor for NpmPackageInfoExtractor {
    fn get_name(&self) -> &str {
        return self.npm_package.name.as_str();
    }

    fn get_version(&self) -> &str {
        return self.npm_package.version.as_str();
    }
}

#[test]
fn test_from_package_path_no_folder_exists() {
    let non_existing_package_path = PathBuf::from("does-not-exist");
    let result = NpmPackageInfoExtractor::from_package_path(&non_existing_package_path);
    assert!(result.is_err());
}

#[test]
fn test_from_package_path_invalid_package_json() {
    let invalid_package_json_lib_path = PathBuf::from(file!()).parent().unwrap().join("./fixures/invalid-package-json-lib");
    let result = NpmPackageInfoExtractor::from_package_path(&invalid_package_json_lib_path);
    assert!(result.is_err());
}

#[test]
fn test_from_package_path_success() {
    let lib1_path = crate::testing::utils::get_repo_root().join("./examples/basic-sample/packages/node-lib1");
    let result = NpmPackageInfoExtractor::from_package_path(&lib1_path).unwrap();
    assert_eq!(result.npm_package.name, "node-lib1");
}