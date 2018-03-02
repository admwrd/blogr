
use std::path::{Path, PathBuf};
use std::env;

#[derive(Debug, Clone)]
pub enum TemplateBody {
    General(String),
    Login(String, Option<String>), // form action url and an optional username
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemplateMenu {
    #[serde(default)]
    pub separator: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
}













