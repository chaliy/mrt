use anyhow::{Result, Context};
use crate::runners::{
    ScriptRunner,
    CommandRunner,
    ScriptRunContext, 
    ScriptRunResult
};

pub struct NpmPackageScriptRunner {
    npm_runner: CommandRunner
}

impl NpmPackageScriptRunner {
    pub fn new() -> NpmPackageScriptRunner {
        return NpmPackageScriptRunner {
            npm_runner: CommandRunner::new("npm".to_string())
        };
    }

    fn list_available_scripts(&self, context: &ScriptRunContext) -> Result<Vec<(String, String)>> {
        
        let json = self.npm_runner.exec_command_json(
            vec!["run".to_string(), "--json".to_string()],
            context
        )?;

        Ok(json
            .as_object()
            .context("`npm run --json` did not return an object")?
            .iter()
            .map(|(key, value)| {
                return (key.to_string(), value.to_string());
            }).collect())
    }
}

impl ScriptRunner for NpmPackageScriptRunner {
    fn run_script(&self, context: &ScriptRunContext) -> Result<ScriptRunResult> {
        return self.npm_runner.run_script(
            vec![
                "run".to_string(), 
                context.script_spec.to_string()
            ],
            context
        );
    }

    fn can_run_script(&self, context: &ScriptRunContext) -> Result<bool> {
        let available_scripts = self.list_available_scripts(context)?;

        Ok(available_scripts
            .iter()
            .any(|(key, _value)| {
                return key == &context.script_spec;
            }))
    }
}