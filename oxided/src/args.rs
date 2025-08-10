use std::path::PathBuf;

use clap::ArgAction;
use clap::Parser;

use crate::config::Device;

#[derive(Parser, Debug)]
#[command(
    version,
    name = "Oxided",
    about = "Run the Oxided server",
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long, value_name = "config.toml", env = "OXIDED_CONFIG")]
    pub config: Option<PathBuf>,

    /// Device to run on: cpu or cuda
    #[arg(long, value_enum, default_value_t = Device::Cpu, env = "OXIDED_DEVICE", hide_possible_values = true)]
    pub device: Device,

    #[arg(short, long, action = ArgAction::SetTrue, env = "OXIDED_VERBOSE")]
    pub verbose: bool,

    #[arg(short = 'h', long = "help", action = ArgAction::Help)]
    _help: (),

    #[arg(long = "version", action = ArgAction::Version)]
    _version: (),
}
