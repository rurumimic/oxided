use std::path::PathBuf;

use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "OXIDED", version, about)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short, long, env = "OXIDED_CONFIG", value_name = "config.toml")]
    pub config: Option<PathBuf>,

    #[arg(short, long, action = ArgAction::SetTrue, env = "OXIDED_VERBOSE")]
    pub verbose: bool,
}
