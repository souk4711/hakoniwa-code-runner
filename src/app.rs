use anyhow::Result;
use std::{
    collections::{hash_map::Values, HashMap},
    str,
};

use crate::{contrib, AppConfig, Executor};
use hakoniwa::{Sandbox, SandboxPolicy};

#[derive(Default)]
pub struct App {
    pub(crate) config: AppConfig,
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
        let sandbox_policy_data = contrib::fs::read_to_string(path)?;
        let sandbox_policy = SandboxPolicy::from_str(&sandbox_policy_data)?;
        let mut sandbox = Sandbox::new();
        sandbox.with_policy(sandbox_policy);
        Ok(sandbox)
    }
}
