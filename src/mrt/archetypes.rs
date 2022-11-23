use std::path::PathBuf;
use anyhow::Result;

use crate::nodejs_npm::NodeJSNpmArchetype;
use crate::python_poetry::PythonPoetryArchetype;
use crate::runners::PackageScriptRunner;

pub trait Archetype {
    fn get_id(&self) -> &str;
    fn matcher(&self, package_path: &PathBuf) -> bool;
    fn get_script_runner(&self) -> Box<dyn PackageScriptRunner>;
    fn get_info_extractor(&self, package_path: &PathBuf) -> Result<Box<dyn crate::package::PackageInfoExtractor>>;
}

fn get_archetypes() -> Vec<Box<dyn Archetype>> {
    vec![
        Box::new(NodeJSNpmArchetype {}),
        Box::new(PythonPoetryArchetype {})
    ]
}

pub fn match_archetype(package_path: &PathBuf) -> Option<Box<dyn Archetype>> {
    let archetypes = get_archetypes();
    for archetype in archetypes {
        if archetype.matcher(package_path) {
            return Some(archetype);
        }
    }
    None
}

pub fn get_archetype_by_id(id: &str) -> Option<Box<dyn Archetype>> {
    let archetypes = get_archetypes();
    for archetype in archetypes {
        if archetype.get_id() == id {
            return Some(archetype);
        }
    }
    None
}