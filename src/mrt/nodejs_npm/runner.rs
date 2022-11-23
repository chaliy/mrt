use std::{process::Command};

use crate::runners::{PackageScriptRunner, PackageScriptRunContext, PackageScriptRunResult, PackageScriptRunResultType};

pub struct NpmPackageScriptRunner {
}

impl PackageScriptRunner for NpmPackageScriptRunner {

    fn run_script(&self, script_spec: &String, context: &PackageScriptRunContext) -> PackageScriptRunResult {
        let context_path = &context.package.path;

        let output = Command::new("npm")
            .arg("run")
            .arg(script_spec)
            .current_dir(context_path.to_str().unwrap_or_default())
            .output()
            .expect("Failed to execute `npm run` command");
 
        PackageScriptRunResult {
            result_type: PackageScriptRunResultType::Success,
            exit_code: output.status.code().unwrap_or_default(),
            stdout: String::from_utf8(output.stdout).unwrap_or_default(),
            stderr: String::from_utf8(output.stderr).unwrap_or_default()
        }
    }
}