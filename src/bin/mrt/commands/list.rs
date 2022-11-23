use serde::{Serialize, Deserialize};

use mrt::package::Package;

use super::{CommandResult, CommandContext, CommandExec};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResult {
    packages: Vec<Package>
}

impl CommandResult<ListResult> for ListResult {
    fn get_result(&self) -> &ListResult {
        &self
    }
}

pub struct ListCommand {
    pub all: bool,
    pub context: CommandContext
}

impl CommandExec<ListResult> for ListCommand {
    fn exec(&self) -> Box<dyn CommandResult<ListResult>> {

        let packages = self.context.project.get_packages(self.all);

        let result = ListResult {
            packages
        };

        return Box::from(result);
    }
}
