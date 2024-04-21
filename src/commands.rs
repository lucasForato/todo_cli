use clap::{Parser, Subcommand};

pub mod list;
use list::ListArgs;

pub mod init;

#[derive(Parser, Debug)]
#[command(version = "0.0.1")]
#[command(
    about = "Work with TODO lists on your terminal",
    long_about = "This CLI is used to created and manipulate TODO lists"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Init,
    List(ListArgs),
}
