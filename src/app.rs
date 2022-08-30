use anyhow::{anyhow, bail, Result};
use std::{
    collections::{hash_map::Values, HashMap},
    fs, str,
};

use crate::{AppConfig, Embed, Executor};
use hakoniwa::{Sandbox, SandboxPolicy};

#[derive(Default)]
pub struct App {
    config: AppConfig,
    executor_map: HashMap<String, Executor>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        for (k, v) in self.config.languages.iter() {
            let mut e = Executor::new(k, &v.name);

            if let Some(compile) = &v.compile {
                let command = &compile.command;
                let sandbox = Self::build_sandbox(&compile.sandbox)?;
                e.with_compile_command(command, sandbox);
            }

            let execute = &v.execute;
            let command = &execute.command;
            let sandbox = Self::build_sandbox(&execute.sandbox)?;
            e.with_execute_command(command, sandbox);

            self.executor_map.insert(k.to_string(), e);
        }
        Ok(())
    }

    pub fn executors(&self) -> Values<String, Executor> {
        self.executor_map.values()
    }

    pub fn get_executor(&self, lang: &str) -> Option<&Executor> {
        self.executor_map.get(lang)
    }

    fn build_sandbox(path: &str) -> Result<Sandbox> {
        let sandbox_policy_data = match path.strip_prefix("embed://") {
            Some(embed_path) => {
                let f = Embed::get(embed_path).ok_or_else(|| anyhow!("{}: No such file", path))?;
                str::from_utf8(&f.data).unwrap().to_string()
            }
            None => match fs::read_to_string(path) {
                Ok(val) => val,
                Err(err) => bail!("{}: {}", path, err.to_string()),
            },
        };
        let sandbox_policy = SandboxPolicy::from_str(&sandbox_policy_data)?;
        let mut sandbox = Sandbox::new();
        sandbox.with_policy(sandbox_policy);
        Ok(sandbox)
    }
}
