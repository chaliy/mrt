use clap::Args;
use serde::{Deserialize, Serialize};
use tabled::{Style, Table};

use mrt::package::Package;

use super::{CommandExec, CommandResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResult {
    packages: Vec<Package>,
}

impl CommandResult<ListResult> for ListResult {
    fn get_result(&self) -> &ListResult {
        self
    }
}

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long, default_value_t = false)]
    pub all: bool,
}

impl CommandExec<ListResult> for ListArgs {
    fn exec(
        &self,
        context: &impl super::CommandExecutionContext,
    ) -> Box<dyn CommandResult<ListResult>> {
        let packages = context.get_project().get_packages(self.all);

        let result = ListResult { packages };

        if context.get_cli().is_interactive() {
            let mut table = Table::new(&result.packages);
            table.with(Style::blank());

            println!("{}", table);
        };

        Box::from(result)
    }
}
