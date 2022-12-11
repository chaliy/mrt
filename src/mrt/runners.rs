use std::{io::{BufReader, BufRead}, process::{Stdio, Command}};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::progress::ProgressReporter;

use super::package::Package;

pub struct ScriptRunContext<'a> {
    pub script_spec: &'a str,
    pub package: &'a Package,
    pub reporter: &'a dyn ProgressReporter
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ScriptRunResultType {
    Success,
    Error(String),
    Noop
}

impl ScriptRunResultType {
    pub fn is_success(&self) -> bool {
        match self {
            ScriptRunResultType::Success => true,
            _ => false
        }
    }

    pub fn is_noop(&self) -> bool {
        match self {
            ScriptRunResultType::Noop => true,
            _ => false
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptRunResult {
    pub command: String,
    pub result_type: ScriptRunResultType,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String
}


pub trait ScriptRunner {
    fn can_run_script(&self, context: &ScriptRunContext) -> Result<bool>;
    fn run_script(&self, context: &ScriptRunContext) -> Result<ScriptRunResult>;
}

impl ScriptRunResult {
    pub fn noop() -> ScriptRunResult {
        return ScriptRunResult {
            command: String::default(),
            result_type: ScriptRunResultType::Noop,
            exit_code: 0,
            stdout: String::from(""),
            stderr: String::from("")
        };
    }
}


pub struct NoopScriptRunner {
}

impl ScriptRunner for NoopScriptRunner {
    fn can_run_script(&self, _context: &ScriptRunContext) -> Result<bool> {
        Ok(false)
    }

    fn run_script(&self, _context: &ScriptRunContext) -> Result<ScriptRunResult> {
        Ok(ScriptRunResult::noop())
    }
}

pub (crate) struct CommandRunner{
    program: String
}

impl CommandRunner {
    pub fn new(program: String) -> Self {
        return Self {
            program
        };
    }

    pub fn can_run_script_dry_run(&self, dry_run_args: Vec<String>, context: &ScriptRunContext) -> Result<bool> {
        let context_path = &context.package.absolute_path;
        let output = Command::new(&self.program)
            .args(dry_run_args)
            .current_dir(context_path)
            .output()?;

        return Ok(match output.status.code() {
            Some(0) => true,
            _ => false
        });
    }

    pub fn exec_command_json(&self, args: Vec<String>, context: &ScriptRunContext) -> Result<Value> {

        let context_path = &context.package.absolute_path;
        let output = Command::new(&self.program)
            .args(args)
            .current_dir(context_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let stdout = String::from_utf8(output.stdout).unwrap_or_default();

        let json: Value = serde_json::from_str(&stdout)?;

        Ok(json)
    }

    pub fn run_script(&self, run_script_args: Vec<String>, context: &ScriptRunContext) -> Result<ScriptRunResult> {
        let command_desc =format!("{} {}", 
            self.program, 
            run_script_args.join(" "));
    
        context.reporter.report_output(&command_desc);
        
        let context_path = &context.package.absolute_path;
        let mut child = Command::new(&self.program)
            .args(run_script_args)
            .current_dir(context_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
    
    
        let stdout = child.stdout.take()
            .with_context(|| format!("Failed to capture standard output for command {}", command_desc))?;
            
        let reader = BufReader::new(stdout);
        
        let mut stdout_lines = String::default();

        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                if line.len() > 0 {
                    context.reporter.report_output(&format!("{}: {}", command_desc, line));
                }
                stdout_lines.push_str(&line);
            });

        let output = child.wait_with_output()?;

        let result_type = match output.status.code() {
            Some(0) => ScriptRunResultType::Success,
            Some(exit_code) => ScriptRunResultType::Error(format!("Exit code {exit_code}")),
            None => ScriptRunResultType::Noop
        };
 
        Ok(ScriptRunResult {
            command: command_desc,
            result_type,
            exit_code: output.status.code().unwrap_or_default(),
            stdout: stdout_lines,
            stderr: String::from_utf8(output.stderr).unwrap_or_default()
        })
    }
}


pub struct MakeScriptRunner {
    make_runner: CommandRunner
}

impl MakeScriptRunner {
    pub fn new() -> Self {
        return Self {
            make_runner: CommandRunner::new("make".to_string())
        };
    }
}


impl ScriptRunner for MakeScriptRunner {
    fn run_script(&self, context: &ScriptRunContext) -> Result<ScriptRunResult> {
        return self.make_runner.run_script(
            vec![context.script_spec.to_string()],
            context
        );
    }

    fn can_run_script(&self, context: &ScriptRunContext) -> Result<bool> {
        return self.make_runner.can_run_script_dry_run(
            vec![
                context.script_spec.to_string(), 
                "--dry-run".to_string()
            ],
            context
        );
    }
}

pub struct WrapperScriptRunner {
    runners: Vec<Box<dyn ScriptRunner>>
}

impl WrapperScriptRunner {
    pub fn wrap_with_generic_runners(runner: Box<dyn ScriptRunner>) -> Self {
        // Runners should be from generic to specific
        // So for example `make` is considered to be more generic
        return Self {
            runners: vec![
                Box::new(MakeScriptRunner::new()),
                runner
            ]
        }
    }

    pub fn generic_runners() -> Self {
        return Self {
            runners: vec![
                Box::new(MakeScriptRunner::new())
            ]
        }
    }
}

impl ScriptRunner for WrapperScriptRunner {
    fn can_run_script(&self, context: &ScriptRunContext) -> Result<bool> {
        for runner in &self.runners {
            if runner.can_run_script(context)? {
                return Ok(true);
            }
        }

        return Ok(false);
    }

    fn run_script(&self, context: &ScriptRunContext) -> Result<ScriptRunResult> {
        for runner in &self.runners {
            if runner.can_run_script(context)? {
                return runner.run_script(context);
            }
        }

        return Ok(ScriptRunResult::noop())
    }
}

#[test]
fn test_make_script_runner() -> anyhow::Result<()> {
    let project_path = crate::testing::utils::get_repo_root().join("./references/basic-sample/mrt.yml");
    let project = crate::project::Project::read(Some(project_path))?;
    let package = project.read_package(std::path::PathBuf::from("./packages/make-lib5"))?;
    let runner = MakeScriptRunner::new();
    let context = ScriptRunContext {
        script_spec: "format",
        package: &package,
        reporter: &crate::progress::LogProgressReporter{}
    };

    assert!(runner.can_run_script(&context)?);

    let result = runner.run_script(&context)?;

    assert!(result.result_type.is_success(), "stderr: {:?}", result.stderr);

    Ok(())
}

#[test]
fn test_make_script_runner_run_no_script() -> anyhow::Result<()> {
    let project_path = crate::testing::utils::get_repo_root().join("./references/basic-sample/mrt.yml");
    let project = crate::project::Project::read(Some(project_path))?;
    let package = project.read_package(std::path::PathBuf::from("./packages/make-lib5"))?;
    let runner = MakeScriptRunner::new();
    let context = ScriptRunContext {
        script_spec: "no-such-script",
        package: &package,
        reporter: &crate::progress::LogProgressReporter{}
    };
    assert!(!runner.can_run_script(&context)?);

    Ok(())
}