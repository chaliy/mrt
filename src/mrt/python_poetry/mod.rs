use std::{path::PathBuf, fs};

use anyhow::Context;

use crate::{
    archetypes::Archetype, 
    runners::NoopPackageScriptRunner, 
    package::PackageInfoExtractor
};

use self::info::PoetryPackageInfoExtractor;

mod info;

pub struct PythonPoetryArchetype {}

impl Archetype for PythonPoetryArchetype {
    fn get_id(&self) -> &str {
        "python/poetry"
    }

    fn matcher(&self, path: &PathBuf) -> bool {
        let pyproject = path.join("pyproject.toml");

        if pyproject.exists() {
            let contents = fs::read_to_string(pyproject).unwrap_or_default();

            return contents.contains("[tool.poetry]");
        }
        return false;
    }

    fn get_script_runner(&self) -> Box<dyn crate::runners::PackageScriptRunner> {
        Box::from(NoopPackageScriptRunner {})
    }

    fn get_info_extractor(&self, package_path: &PathBuf) -> anyhow::Result<Box<dyn PackageInfoExtractor>> {
        let extractor = PoetryPackageInfoExtractor::from_package_path(package_path)
            .context(format!("Get information extractor for package {}", package_path.display()))?;

        return Ok(Box::from(extractor));
    }
}