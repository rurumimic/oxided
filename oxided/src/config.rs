use clap::ValueEnum;

use crate::args::Cli;

#[derive(Debug)]
pub struct Config {
    pub model: Option<ModelConfig>,
    pub device: Device,
    pub verbose: bool,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Device {
    Cpu,
    Cuda,
}

#[derive(Debug)]
pub struct ModelConfig {
    pub path: Option<String>,
    pub config_path: Option<String>,
    pub tokenizer_path: Option<String>,
    pub weight_path: Option<String>,
}

impl Config {
    pub fn merge(mut self, cli: &Cli) -> Self {
        self.device = cli.device;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: None,
            device: Device::Cpu,
            verbose: false,
        }
    }
}
