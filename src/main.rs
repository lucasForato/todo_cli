use clap::Parser;

mod commands;
use commands::{Cli, Commands};

use commands::init;
use commands::list;

fn main() {
    let args = Cli::parse();

    match args.cmd {
        Commands::Init => {
            let result = init::execute();
            match result {
                Ok(_) => println!("Initialization successful!"),
                Err(err) => eprintln!("Error during initialization:: {}", err),
            }
        }
        Commands::List(args) => {
            let result = list::execute(args);
            match result {
                Ok(_) => (),
                Err(err) => eprintln!("Error during list command:: {}", err),
            }
        }
    }
}
