use std::{path::PathBuf, fs};

use anyhow::{Result, Context};

use crate::{
    archetypes::Archetype, 
    runners::WrapperScriptRunner, 
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

    fn get_script_runner(&self) -> Box<dyn crate::runners::ScriptRunner> {
        Box::from(WrapperScriptRunner::generic_runners())
    }

    fn get_info_extractor(&self, package_path: &PathBuf) -> Result<Box<dyn PackageInfoExtractor>> {
        let extractor = PoetryPackageInfoExtractor::from_package_path(package_path)
            .context(format!("Get information extractor for package {}", package_path.display()))?;

        return Ok(Box::from(extractor));
    }
}


#[test]
fn test_script_runner_make() -> anyhow::Result<()> {
    let project_path = crate::testing::utils::get_repo_root().join("./references/basic-sample/mrt.yml");
    let project = crate::project::Project::read(Some(project_path))?;
    let package = project.read_package(std::path::PathBuf::from("./packages/py-lib2"))?;

    let archetype = PythonPoetryArchetype{};

    let runner = archetype.get_script_runner();

    let context = crate::runners::ScriptRunContext {
        script_spec: "format",
        package: &package,
        reporter: &crate::progress::LogProgressReporter{}
    };

    assert!(runner.can_run_script(&context)?);

    let result = runner.run_script(&context)?;

    assert!(result.result_type.is_success(), "stderr: {:?}", result.stderr);

    Ok(())
}