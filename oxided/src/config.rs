use serde::Deserialize;
use std::path::PathBuf;

use crate::device::Device;

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    pub device: Option<Device>,
    pub verbose: Option<bool>,

    #[serde(default)]
    pub model: AppModelConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct AppModelConfig {
    pub path: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub tokenizer_path: Option<PathBuf>,
    pub weight_path: Option<PathBuf>,
}
