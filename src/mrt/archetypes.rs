use anyhow::Result;
use std::path::Path;

use crate::nodejs_npm::NodeJSNpmArchetype;
use crate::python_poetry::PythonPoetryArchetype;
use crate::runners::ScriptRunner;

pub trait Archetype {
    fn get_id(&self) -> &str;
    fn matcher(&self, package_path: &Path) -> bool;
    fn get_script_runner(&self) -> Box<dyn ScriptRunner>;
    fn get_info_extractor(
        &self,
        package_path: &Path,
    ) -> Result<Box<dyn crate::package::PackageInfoExtractor>>;
}

fn get_archetypes() -> Vec<Box<dyn Archetype>> {
    vec![
        Box::new(NodeJSNpmArchetype {}),
        Box::new(PythonPoetryArchetype {}),
    ]
}

pub fn detect_archetype(package_path: &Path) -> Option<Box<dyn Archetype>> {
    get_archetypes()
        .into_iter()
        .find(|archetype| archetype.matcher(package_path))
}

pub fn get_archetype_by_id(id: &str) -> Option<Box<dyn Archetype>> {
    get_archetypes()
        .into_iter()
        .find(|archetype| archetype.get_id() == id)
}
