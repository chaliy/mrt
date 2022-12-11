#[cfg(test)]
pub mod testing {

    use std::{path::PathBuf, env};
    use crate::{Cli, commands::{list::ListArgs, run_script::RunScriptArgs}};

    fn get_repo_root() -> PathBuf {
        let mut path = env::current_dir().unwrap();
        while !path.join(".git").exists() {
            path = path.parent().unwrap().to_path_buf();
        }
        path
    }

    #[test]
    fn test_list_basic_sample() -> anyhow::Result<()> {
        let manifest = get_repo_root().join("./references/basic-sample/mrt.yml");
        let cli = Cli{
            command: None,
            manifest: Some(manifest),
            output: None
        };

        cli.exec_command(&ListArgs{
            all: false
        });

        Ok(())
    }

    #[test]
    fn test_run_script_format_basic_sample() -> anyhow::Result<()> {
        let manifest = get_repo_root().join("./references/basic-sample/mrt.yml");
        let cli = Cli{
            command: None,
            manifest: Some(manifest),
            output: None
        };

        cli.exec_command(&RunScriptArgs{
            script_spec: "format".to_string(),
        });

        Ok(())
    }

    #[test]
    fn test_run_script_build_basic_sample() -> anyhow::Result<()> {
        let manifest = get_repo_root().join("./references/basic-sample/mrt.yml");
        let cli = Cli{
            command: None,
            manifest: Some(manifest),
            output: None
        };

        cli.exec_command(&RunScriptArgs{
            script_spec: "build".to_string(),
        });

        Ok(())
    }
}