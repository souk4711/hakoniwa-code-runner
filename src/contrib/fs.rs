use anyhow::{anyhow, bail, Result};
use std::{fs, str};

pub fn read_to_string(path: &str) -> Result<String> {
    match path.strip_prefix("embed://") {
        Some(embed_path) => {
            let f =
                crate::Embed::get(embed_path).ok_or_else(|| anyhow!("{}: No such file", path))?;
            Ok(str::from_utf8(&f.data).unwrap().to_string())
        }
        None => match fs::read_to_string(path) {
            Ok(val) => Ok(val),
            Err(err) => bail!("{}: {}", path, err.to_string()),
        },
    }
}
