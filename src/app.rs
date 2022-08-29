use anyhow::Result;
use std::collections::{hash_map::Values, HashMap};

use crate::{AppConfig, Executor};

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
            let executor = Executor::new(k, &v.name);
            self.executor_map.insert(k.to_string(), executor);
        }
        Ok(())
    }

    pub fn executors(&self) -> Values<String, Executor> {
        self.executor_map.values()
    }
}
