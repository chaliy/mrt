use std::thread;
use std::time::Duration;

use clap::Args;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use console::style;
use serde::{Serialize, Deserialize};

use mrt::archetypes::get_archetype_by_id;
use mrt::progress::ProgressReporter;
use mrt::{package::Package};
use mrt::runners::{PackageScriptRunResult, PackageScriptRunContext, PackageScriptRunResultType};

use super::{CommandResult, ProgressBarReporter, CommandExec, NoopProgressReporter};

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageResult<TResult> {
    pub package: Package,
    pub result: TResult
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunScriptResult {
    results: Vec<PackageResult<PackageScriptRunResult>>
}

impl CommandResult<RunScriptResult> for RunScriptResult {
    fn get_result(&self) -> &RunScriptResult {
        &self
    }
}

#[derive(Args)]
pub struct RunScriptArgs {
    /// Name of the script to run
    #[arg(index = 1)]
    pub script_spec: String
}

fn exec_package(package: &Package, script_spec: &str, reporter: &impl ProgressReporter) -> PackageScriptRunResult {
    let script_runner = get_archetype_by_id(package.archetype_id.as_str())
        .unwrap()
        .get_script_runner();

    let result = script_runner.run_script(script_spec, &PackageScriptRunContext {
        package,
        reporter
    });

    return result;
}

impl RunScriptArgs {

    fn exec_interactive(&self, packages: &Vec<Package>) -> Vec<PackageResult<PackageScriptRunResult>> {
        let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
        let multi_progress = MultiProgress::new();

        return packages
            .iter()
            .map(|package| {
                let package = package.clone();
                let script_spec = self.script_spec.clone();

                let progress_bar = multi_progress.add(ProgressBar::new_spinner());
                progress_bar.set_style(spinner_style.clone());
                progress_bar.set_prefix(format!("[{}]", package.name));
                progress_bar.enable_steady_tick(Duration::from_millis(100));

                return thread::spawn(move || {

                    let reporter = ProgressBarReporter{
                        progress_bar: &progress_bar
                    };
    
                    let result = exec_package(&package, &script_spec, &reporter);

                    match result.result_type {
                        PackageScriptRunResultType::Success => {
                            progress_bar.finish_with_message(format!("{}: {} ✨", result.command, style("Done").green()));
                        },
                        PackageScriptRunResultType::Error(ref message) => {
                            progress_bar.finish_with_message(format!("{}: {} ❌\n{}", result.command, style(message).red(), result.stderr));
                        },
                        PackageScriptRunResultType::Noop => {
                            progress_bar.finish_with_message(format!("Skipped! ⏭️"));
                        }
                    }

                    return PackageResult {
                        package,
                        result
                    };
                });
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect();
    }

    fn exec_non_interactive(&self, packages: &Vec<Package>) -> Vec<PackageResult<PackageScriptRunResult>> {

        return packages
            .iter()
            .map(|package| {
                let package = package.clone();
                let script_spec = self.script_spec.clone();

                return thread::spawn(move || {
                
                    let reporter = NoopProgressReporter{};

                    let result = exec_package(&package, &script_spec, &reporter);

                    return PackageResult {
                        package: package.clone(),
                        result
                    };
                });
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect();

    }
}

impl CommandExec<RunScriptResult> for RunScriptArgs {
    fn exec(&self, context: &impl super::CommandExecutionContext) -> Box<dyn CommandResult<RunScriptResult>> {
        let packages = context.get_project().get_packages(false);
        
        let results = match context.get_cli().is_interactive() {
            true => self.exec_interactive(&packages),
            false => self.exec_non_interactive(&packages)
        };

        return Box::from(RunScriptResult{
            results
        });
    }
}