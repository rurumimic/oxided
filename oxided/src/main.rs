use clap::Parser;
use oxided::args::Cli;
use oxided::config::Config;

fn main() {
    let config = Config::default();
    let cli = Cli::parse();

    println!("Config: {:#?}", config);
    println!("CLI: {:#?}", cli);

    let config = config.merge(&cli);
    println!("Config: {:#?}", config);

    println!("Oxided");
}
