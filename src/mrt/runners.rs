use serde::{Serialize, Deserialize};

use crate::progress::ProgressReporter;

use super::package::Package;

pub struct PackageScriptRunContext<'a> {
    pub package: &'a Package,
    pub reporter: &'a dyn ProgressReporter
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PackageScriptRunResultType {
    Success,
    Error(String),
    Noop
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageScriptRunResult {
    pub command: String,
    pub result_type: PackageScriptRunResultType,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String
}

pub trait PackageScriptRunner {
    fn run_script(&self, script_spec: &str, context: &PackageScriptRunContext) -> PackageScriptRunResult;
}

pub struct NoopPackageScriptRunner {
}

impl PackageScriptRunner for NoopPackageScriptRunner {
    fn run_script(&self, _script_spec: &str, _context: &PackageScriptRunContext) -> PackageScriptRunResult {
        PackageScriptRunResult {
            command: String::default(),
            result_type: PackageScriptRunResultType::Noop,
            exit_code: 0,
            stdout: String::from(""),
            stderr: String::from("")
        }
    }
}