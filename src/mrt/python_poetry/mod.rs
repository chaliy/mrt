use std::{path::PathBuf, fs};

use crate::{archetypes::Archetype, runners::NoopPackageScriptRunner, package::{NoopPackageInfoExtractor, PackageInfoExtractor}};

pub struct PythonPoetryArchetype {}

impl Archetype for PythonPoetryArchetype {
    fn get_id(&self) -> &str {
        "python/poetry"
    }

    fn matcher(&self, path: &PathBuf) -> bool {
        let pyproject = path.join("pyproject.toml");

        if pyproject.exists() {
            let contents = fs::read_to_string(pyproject)
                .expect("Should have been able to read the file");

            return contents.contains("[tool.poetry]");
        }
        return false;
    }

    fn get_script_runner(&self) -> Box<dyn crate::runners::PackageScriptRunner> {
        Box::from(NoopPackageScriptRunner {})
    }

    fn get_info_extractor(&self, _package_path: &PathBuf) -> anyhow::Result<Box<dyn PackageInfoExtractor>> {
        Ok(Box::from(NoopPackageInfoExtractor {}))
    }
}