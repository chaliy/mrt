use serde::{Serialize, Deserialize};

use mrt::package::Package;
use mrt::runners::PackageScriptRunResult;

use super::{CommandResult, CommandContext, CommandExec};

#[derive(Serialize, Deserialize, Debug)]
pub struct RunScriptResult {
    results: Vec<PackagePackageScriptRunResult>
}

impl CommandResult<RunScriptResult> for RunScriptResult {
    fn get_result(&self) -> &RunScriptResult {
        &self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackagePackageScriptRunResult {
    package: Package,
    result: PackageScriptRunResult
}

pub struct RunScriptCommand {
    pub script_spec: String,
    pub context: CommandContext
}

impl CommandExec<RunScriptResult> for RunScriptCommand {
    fn exec(&self) -> Box<dyn CommandResult<RunScriptResult>> {

        let mut results = Vec::new();

        let packages = self.context.project.get_packages(false);

        for package in packages {
            let result = package.run_script(&self.script_spec);
            results.push(PackagePackageScriptRunResult {
                package: package.clone(),
                result: result
            });
        }

        return Box::from(RunScriptResult{
            results
        });
    }
}
