use std::path::PathBuf;

use clap::ArgAction;
use clap::Args;
use clap::Parser;
use clap::builder::BoolishValueParser;
use serde::Deserialize;

use crate::config::AppConfig;
use crate::config::AppModelConfig;
use crate::device::Device;

#[derive(Parser, Debug, Deserialize)]
#[command(
    version,
    name = "Oxided",
    about = "Run the Oxided server",
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Arguments {
    /// Device to run on: cpu or cuda
    #[arg(
        long,
        value_enum,
        ignore_case = true,
        env = "OXIDED_DEVICE",
        hide_possible_values = true
    )]
    pub device: Option<Device>,

    /// Path to the configuration file
    #[arg(short, long, value_name = "config.toml", env = "OXIDED_CONFIG")]
    pub config: Option<PathBuf>,

    #[command(flatten)]
    pub model: ModelConfig,

    #[arg(short, long, env = "OXIDED_VERBOSE", value_parser = BoolishValueParser::new(), num_args = 0..=1, default_missing_value = "true")]
    pub verbose: Option<bool>,

    #[arg(long = "version", action = ArgAction::Version)]
    _version: (),

    #[arg(short = 'h', long = "help", action = ArgAction::Help)]
    _help: (),
}

impl Arguments {
    pub fn load_config(self) -> Result<AppConfig, String> {
        let file = if let Some(config_path) = &self.config {
            if !config_path.exists() {
                return Err(format!(
                    "Configuration file {} does not exist",
                    config_path.display()
                ));
            }
            let s = std::fs::read_to_string(config_path)
                .map_err(|e| format!("Failed to read configuration file: {e}"))?;
            toml::from_str(&s).map_err(|e| {
                format!(
                    "Failed to parse configuration file {}: {e}",
                    config_path.display()
                )
            })?
        } else {
            AppConfig::default()
        };

        let device = Some(self.device.or(file.device).unwrap_or(Device::Cpu));
        let verbose = self.verbose.or(file.verbose);
        let model = AppModelConfig {
            path: self.model.path.or(file.model.path),
            config_path: self.model.config_path.or(file.model.config_path),
            tokenizer_path: self.model.tokenizer_path.or(file.model.tokenizer_path),
            weight_path: self.model.weight_path.or(file.model.weight_path),
        };

        Ok(AppConfig {
            device,
            verbose,
            model,
        })
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            device: Some(Device::Cpu),
            verbose: Some(false),
            config: None,
            model: ModelConfig {
                path: None,
                config_path: None,
                tokenizer_path: None,
                weight_path: None,
            },
            _version: (),
            _help: (),
        }
    }
}

#[derive(Debug, Args, Deserialize)]
pub struct ModelConfig {
    #[arg(
        long = "model-path",
        value_name = "/path/to/model/",
        env = "OXIDED_MODEL_PATH"
    )]
    pub path: Option<PathBuf>,

    #[arg(
        long = "model-config-path",
        value_name = "config.json",
        env = "OXIDED_MODEL_CONFIG_PATH"
    )]
    pub config_path: Option<PathBuf>,

    #[arg(
        long = "tokenizer-path",
        value_name = "tokenizer.json",
        env = "OXIDED_TOKENIZER_PATH"
    )]
    pub tokenizer_path: Option<PathBuf>,

    #[arg(
        long = "model-weight-path",
        value_name = "model.safetensors",
        env = "OXIDED_MODEL_WEIGHT_PATH"
    )]
    pub weight_path: Option<PathBuf>,
}
