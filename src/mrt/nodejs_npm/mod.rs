use std::path::Path;

use anyhow::{Context, Result};

use crate::{archetypes::Archetype, package::PackageInfoExtractor, runners::WrapperScriptRunner};

use self::{info::NpmPackageInfoExtractor, runner::NpmPackageScriptRunner};

mod info;
mod runner;

pub struct NodeJSNpmArchetype {}

impl Archetype for NodeJSNpmArchetype {
    fn get_id(&self) -> &str {
        "nodejs/npm"
    }

    fn matcher(&self, package_path: &Path) -> bool {
        package_path.join("package.json").exists()
    }

    fn get_script_runner(&self) -> Box<dyn crate::runners::ScriptRunner> {
        Box::from(WrapperScriptRunner::wrap_with_generic_runners(Box::from(
            NpmPackageScriptRunner::new(),
        )))
    }

    fn get_info_extractor(&self, package_path: &Path) -> Result<Box<dyn PackageInfoExtractor>> {
        let extractor =
            NpmPackageInfoExtractor::from_package_path(package_path).context(format!(
                "Get information extractor for package {}",
                package_path.display()
            ))?;

        Ok(Box::from(extractor))
    }
}
