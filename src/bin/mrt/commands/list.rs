use clap::Args;
use serde::{Serialize, Deserialize};
use tabled::{Table, Style};

use mrt::package::Package;

use super::{CommandResult, CommandExec};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResult {
    packages: Vec<Package>
}

impl CommandResult<ListResult> for ListResult {
    fn get_result(&self) -> &ListResult {
        &self
    }
}

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long, default_value_t=false)]
    pub all: bool
}

impl CommandExec<ListResult> for ListArgs {
    fn exec(&self, context: &impl super::CommandExecutionContext) -> Box<dyn CommandResult<ListResult>> {

        let packages = context.get_project().get_packages(self.all);

        let result = ListResult { packages };

        match context.get_cli().is_interactive() {
            true => {
                let mut table = Table::new(&result.packages);
                table
                    .with(Style::blank());

                println!("{}", table);
            },
            false => ()
        };

        return Box::from(result);
    }
}
