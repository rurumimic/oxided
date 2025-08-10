use clap::Parser;
use oxided::args::Cli;

fn main() {
    let cli = Cli::parse();

    println!("CLI: {:#?}", cli);
    println!("Config: {:?}", cli.config);

    println!("Oxided");
}
