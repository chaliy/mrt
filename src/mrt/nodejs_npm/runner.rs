use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

use crate::runners::{
    PackageScriptRunner, 
    PackageScriptRunContext, 
    PackageScriptRunResult, 
    PackageScriptRunResultType
};

pub struct NpmPackageScriptRunner {
}

impl PackageScriptRunner for NpmPackageScriptRunner {

    fn run_script(&self, script_spec: &str, context: &PackageScriptRunContext) -> PackageScriptRunResult {
        let command = format!("npm run {}", script_spec);
        let context_path = &context.package.absolute_path;

        context.reporter.report_output(&format!("{}", command));
    
        let mut child = Command::new("npm")
            .arg("run")
            .arg(script_spec)
            .current_dir(context_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().unwrap();

        let stdout = child.stdout.take()
            .expect("Should be able to capture standard output");
            
        let reader = BufReader::new(stdout);
        
        let mut stdout_lines = String::default();

        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                if line.len() > 0 {
                    context.reporter.report_output(&format!("{}: {}", command, line));
                }
                stdout_lines.push_str(&line);
            });

        let output = child.wait_with_output().unwrap();

        let result_type = match output.status.code() {
            Some(0) => {
                PackageScriptRunResultType::Success
            },
            Some(exit_code) => {
                PackageScriptRunResultType::Error(format!("Exit code {exit_code}"))
            },
            None => {
                PackageScriptRunResultType::Noop
            }
        };
 
        PackageScriptRunResult {
            command,
            result_type,
            exit_code: output.status.code().unwrap_or_default(),
            stdout: stdout_lines,
            stderr: String::from_utf8(output.stderr).unwrap_or_default()
        }
    }
}