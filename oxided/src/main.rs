use clap::Parser;

use oxided::arguments::Arguments;

fn main() {
    let args = Arguments::parse();
    let config = args.load_config().unwrap_or_else(|err| {
        eprintln!("Error loading configuration: {}", err);
        std::process::exit(1);
    });
    println!("config: {:#?}", config);

    println!("Oxided");
}
