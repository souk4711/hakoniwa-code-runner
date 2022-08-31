use anyhow::Result;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

use crate::contrib::handlebars::render_to_string_helper;

lazy_static! {
    static ref SANDBOX_POLICY_HANDLEBARS: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("render_to_string", Box::new(render_to_string_helper));
        handlebars
    };
}

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
        let data = SANDBOX_POLICY_HANDLEBARS.render_template(data, &())?;
        let policy: Self = toml::from_str(&data)?;
        Ok(policy)
    }
}
