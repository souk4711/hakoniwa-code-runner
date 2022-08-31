use anyhow::Result;
use chrono::prelude::*;
use serde_variant::to_variant_name;
use std::{fs, path::PathBuf, time::Duration};

use crate::contrib;
use hakoniwa::Sandbox;

pub struct ExecutorFile {
    name: String,
    content: String,
}

impl ExecutorFile {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub struct ExecutorResult {
    pub status: String,
    pub reason: String,
    pub exit_code: Option<i32>,
    pub start_time: Option<DateTime<Utc>>,
    pub real_time: Option<Duration>,
    pub system_time: Option<Duration>,
    pub user_time: Option<Duration>,
    pub max_rss: Option<i64>,
    pub stdout: String,
    pub stderr: String,
}

impl ExecutorResult {
    fn complie_error(r: hakoniwa::ExecutorResult) -> Self {
        let mut r = Self::from(r);
        r.status = String::from("CE");
        r
    }

    fn sandbox_setup_error(reason: &str) -> Self {
        Self {
            status: String::from("SE"),
            reason: reason.to_string(),
            ..Default::default()
        }
    }
}

impl From<hakoniwa::ExecutorResult> for ExecutorResult {
    fn from(r: hakoniwa::ExecutorResult) -> Self {
        Self {
            status: to_variant_name(&r.status).unwrap().to_string(),
            reason: r.reason,
            exit_code: r.exit_code,
            start_time: r.start_time,
            real_time: r.real_time,
            system_time: r.system_time,
            user_time: r.user_time,
            max_rss: r.max_rss,
            stdout: String::from_utf8_lossy(&r.stdout).to_string(),
            stderr: String::from_utf8_lossy(&r.stderr).to_string(),
        }
    }
}

#[derive(Default)]
pub struct Executor {
    pub(crate) id: String,
    pub(crate) name: String,
    work_dir: PathBuf,
    compile: Vec<String>,
    compilebox: Sandbox,
    execute: Vec<String>,
    executebox: Sandbox,
}

impl Executor {
    pub(crate) fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            work_dir: contrib::tmpdir::random_name("hakoniwa-code-runner"),
            ..Default::default()
        }
    }

    pub(crate) fn with_compile_command<SC: AsRef<str>>(
        &mut self,
        command: &[SC],
        sandbox: Sandbox,
    ) {
        self.compile = command.iter().map(|c| String::from(c.as_ref())).collect();
        self.compilebox = sandbox;
    }

    pub(crate) fn with_execute_command<SC: AsRef<str>>(
        &mut self,
        command: &[SC],
        sandbox: Sandbox,
    ) {
        self.execute = command.iter().map(|c| String::from(c.as_ref())).collect();
        self.executebox = sandbox;
    }

    pub fn run(&self, files: &[ExecutorFile]) -> ExecutorResult {
        match self._run(files) {
            Ok(val) => val,
            Err(err) => {
                let err = err.to_string();
                ExecutorResult::sandbox_setup_error(&err)
            }
        }
    }

    fn _run(&self, files: &[ExecutorFile]) -> Result<ExecutorResult> {
        let _work_dir = contrib::tmpdir::new(&self.work_dir)?;
        for f in files {
            fs::write(self.work_dir.join(&f.name), &f.content)?;
        }

        if !self.compile.is_empty() {
            let mut command = self.compilebox.command(&self.compile[0], &self.compile);
            let result = command
                .rw_bind(&self.work_dir, "/hako")?
                .current_dir("/hako")?
                .run();
            match result.exit_code {
                Some(0) => {}
                _ => return Ok(ExecutorResult::complie_error(result)),
            }
        }

        let mut command = self.executebox.command(&self.execute[0], &self.execute);
        let result = command
            .rw_bind(&self.work_dir, "/hako")?
            .current_dir("/hako")?
            .run();
        Ok(ExecutorResult::from(result))
    }
}
