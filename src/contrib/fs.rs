use anyhow::{bail, Result};
use std::{fs, str};

pub fn read_to_string(path: &str) -> Result<String> {
    match fs::read_to_string(path) {
        Ok(val) => Ok(val),
        Err(err) => bail!("{}: {}", path, err.to_string()),
    }
}
