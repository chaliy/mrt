use clap::Args;
use serde::{Deserialize, Serialize};
use tabled::{builder::Builder, settings::Style};

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
            let mut builder = Builder::default();
            builder.push_record(["name", "version", "path", "archetype", "status"]);
            for package in &result.packages {
                builder.push_record([
                    package.name.as_str(),
                    package.version.as_str(),
                    package.path.as_str(),
                    package.archetype_id.as_str(),
                    package.status.to_string().as_str(),
                ]);
            }

            let mut table = builder.build();
            table.with(Style::blank());

            println!("{}", table);
        };

        Box::from(result)
    }
}
