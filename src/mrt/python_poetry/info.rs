use std::{path::PathBuf};
use anyhow::{Result, Context};
use serde::Deserialize;
use toml;

use crate::{package::PackageInfoExtractor};

pub struct PoetryPackageInfoExtractor {
    poetry_pyproject: PoetryPyProject
}

#[derive(Deserialize, Debug)]
struct PyProjectToolPoetry {
    name: String,
    version: String
}

#[derive(Deserialize, Debug)]
struct PyProjectTool {
    poetry: PyProjectToolPoetry
}

// Represents Poetry package info from pyproject.toml
#[derive(Deserialize, Debug)]
struct PoetryPyProject {
    tool: PyProjectTool
}

impl PoetryPackageInfoExtractor {

    pub fn from_package_path(package_path: &PathBuf) -> Result<PoetryPackageInfoExtractor> {

        let pyproject_toml_path = package_path.join("pyproject.toml");

        let pyproject_toml_content = std::fs::read_to_string(&pyproject_toml_path)
            .with_context(|| format!("Failed to read pyproject.toml file at {}", pyproject_toml_path.display()))?;

        return Ok(PoetryPackageInfoExtractor {
            poetry_pyproject:  toml::from_str(&pyproject_toml_content)?
        });
    }
}

impl PackageInfoExtractor for PoetryPackageInfoExtractor {
    fn get_name(&self) -> &str {
        return self.poetry_pyproject.tool.poetry.name.as_str();
    }

    fn get_version(&self) -> &str {
        return self.poetry_pyproject.tool.poetry.version.as_str();
    }
}

#[test]
fn test_from_package_path_no_folder_exists() {
    let non_existing_package_path = PathBuf::from("does-not-exist");
    let result = PoetryPackageInfoExtractor::from_package_path(&non_existing_package_path);
    assert!(result.is_err());
}

#[test]
fn test_from_package_path_success() {
    let lib1_path = crate::testing::utils::get_repo_root().join("./references/basic-sample/packages/py-lib2");
    let result = PoetryPackageInfoExtractor::from_package_path(&lib1_path).unwrap();
    assert_eq!(result.get_name(), "py_lib2");
}