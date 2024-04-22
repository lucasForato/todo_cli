use clap::Parser;

mod utils;
use utils::render_text;
use utils::RendererProps;

mod commands;
use commands::init;
use commands::list;
use commands::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.cmd {
        Commands::Init => match init::execute() {
            Ok(_) => (),
            Err(err) => {
                let err_message = format!("Error during initialization:: {}", err);
                render_text(&err_message, RendererProps::error());
            }
        },
        Commands::List(args) => match list::execute(args) {
            Ok(_) => (),
            Err(err) => {
                let err_message = format!("Error during listing:: {}", err);
                render_text(&err_message, RendererProps::error())
            }
        },
    }
}
