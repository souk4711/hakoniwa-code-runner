use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct LanguageCommandConfig {
    pub(crate) command: Vec<String>,
    pub(crate) sandbox: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct LanguageConfig {
    pub(crate) name: String,
    pub(crate) compile: Option<LanguageCommandConfig>,
    pub(crate) execute: LanguageCommandConfig,
}

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    #[serde(rename = "lang")]
    pub(crate) languages: HashMap<String, LanguageConfig>,
}

impl AppConfig {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(data: &str) -> Result<Self> {
        let policy: Self = toml::from_str(data)?;
        Ok(policy)
    }
}
