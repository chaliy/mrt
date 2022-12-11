use std::path::PathBuf;

use anyhow::{Result,Context};

use crate::{archetypes::Archetype, package::PackageInfoExtractor, runners::WrapperScriptRunner};

use self::{runner::NpmPackageScriptRunner, info::NpmPackageInfoExtractor};

mod runner;
mod info;

pub struct NodeJSNpmArchetype {}

impl Archetype for NodeJSNpmArchetype {
    fn get_id(&self) -> &str {
        "nodejs/npm"
    }

    fn matcher(&self, package_path: &PathBuf) -> bool {
        package_path.join("package.json").exists()
    }

    fn get_script_runner(&self) -> Box<dyn crate::runners::ScriptRunner> {
        Box::from(WrapperScriptRunner::wrap_with_generic_runners(
            Box::from(NpmPackageScriptRunner::new())
        ))
    }

    fn get_info_extractor(&self, package_path: &PathBuf) -> Result<Box<dyn PackageInfoExtractor>> {
        let extractor = NpmPackageInfoExtractor::from_package_path(package_path)
            .context(format!("Get information extractor for package {}", package_path.display()))?;

        return Ok(Box::from(extractor));
    }
}
