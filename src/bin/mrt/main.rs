use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde::ser;
use commands::{CommandContext, list::ListCommand, CommandExec, run_script::RunScriptCommand};
use output::write_command_stdout;
use mrt::project::Project;

mod commands;
mod output;

/// MRT - MonoRepo Tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the manifest file
    #[arg(short, long, default_value_t = String::from("mrt.yml"), global = true)]
    manifest: String,
}

#[derive(Subcommand)]
enum Commands {
    /// do something
    List {
        #[arg(short, long, default_value_t=false)]
        all: bool
    },
    /// run scripts in all monorepo packages
    Run {
        /// Name of the script to run
        #[arg(index = 1)]
        script_spec: String,
    },
}

impl Cli {
    fn get_command_context(&self) -> CommandContext {
        let project = Project::new(PathBuf::from(&self.manifest));
        CommandContext::new(project)
    }

    fn exec_command<T>(&self, command: &dyn CommandExec<T>) 
        where T: ser::Serialize {
        let result = command.exec();

        write_command_stdout(&result);
    }
}



fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List { all }) => {
            cli.exec_command(&ListCommand {
                all: *all,
                context: cli.get_command_context(),
            });
        }
        Some(Commands::Run { script_spec }) => {
            cli.exec_command(&RunScriptCommand {
                context: cli.get_command_context(),
                script_spec: script_spec.clone(),
            });
        }
        None => {}
    }
}
