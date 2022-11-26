use mrt::project::Project;
use serde::ser;
pub mod list;
pub mod run_script;


pub struct CommandContext {
    project: Project
}

impl CommandContext {
    pub fn new(project: Project) -> CommandContext {
        CommandContext {
            project
        }
    }
}

pub trait CommandExec<T> 
    where T: ser::Serialize{
        fn exec(&self) -> Box<dyn CommandResult<T>>;
}

pub trait CommandResult<T> 
    where T: ser::Serialize{
    fn get_result(&self) -> &T;
}