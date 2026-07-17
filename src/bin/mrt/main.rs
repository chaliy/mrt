use clap::{CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{Shell, generate};
use commands::{CommandExec, CommandExecutionContext, list::ListArgs, run_script::RunScriptArgs};
use mrt::project::Project;
use output::write_command_stdout_as_json;
use serde::ser;
use std::path::PathBuf;

mod commands;
mod output;
#[cfg(test)]
mod testing;

/// MRT - MonoRepo Tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the manifest file
    #[arg(short, long, global = true, value_hint = ValueHint::FilePath)]
    manifest: Option<PathBuf>,

    /// Type of the output format
    #[arg(short, long, global = true)]
    output: Option<Output>,
}

#[derive(clap::ValueEnum, Clone)]
enum Output {
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    /// list detected packages
    List(ListArgs),
    /// run scripts in all monorepo packages
    Run(RunScriptArgs),
    /// outputs the completion file for given shell
    Completion {
        #[arg(index = 1, value_enum)]
        shell: Shell,
    },
}

impl Cli {
    fn exec_command<T>(&self, executor: &impl CommandExec<T>)
    where
        T: ser::Serialize,
    {
        let result = executor.exec(self);

        if let Some(Output::Json) = self.output {
            write_command_stdout_as_json(&*result)
        }
    }

    pub fn is_interactive(&self) -> bool {
        !self.output.is_some()
    }
}

impl CommandExecutionContext for Cli {
    fn get_project(&self) -> Project {
        let manifest_path = self.manifest.as_ref().map(PathBuf::from);

        Project::read(manifest_path).expect("Failed to read project manifest")
    }

    fn get_cli(&self) -> &Cli {
        self
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List(args)) => {
            cli.exec_command(args);
        }
        Some(Commands::Run(args)) => {
            cli.exec_command(args);
        }
        Some(Commands::Completion { shell }) => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(*shell, &mut cmd, name, &mut std::io::stdout());
        }
        None => {}
    }
}

#[test]
fn verify_cli() {
    Cli::command().debug_assert();
}
