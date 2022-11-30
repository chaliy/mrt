use indicatif::{ProgressBar};
use serde::ser;

use mrt::{project::Project, progress::ProgressReporter};

use crate::Cli;

pub mod list;
pub mod run_script;

pub trait CommandExecutionContext {
    fn get_project(&self) -> Project;
    fn get_cli(&self) -> &Cli;
}

pub trait CommandExec<T>
    where T: ser::Serialize {
        fn exec(&self, context: &impl CommandExecutionContext) -> Box<dyn CommandResult<T>>;
}

pub trait CommandResult<T> 
    where T: ser::Serialize {
    fn get_result(&self) -> &T;
}

pub(super) struct ProgressBarReporter<'a> {
    pub progress_bar: &'a ProgressBar
}

impl<'a> ProgressReporter for ProgressBarReporter<'a> {
    fn report_output(&self, message: &str) {
        self.progress_bar.set_message(message.to_string());
    }
}

pub(super) struct NoopProgressReporter {
}

impl ProgressReporter for NoopProgressReporter {
    fn report_output(&self, _message: &str) {
    }
}