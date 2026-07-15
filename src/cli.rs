/* Imports */
use clap::{Args, Parser, Subcommand};
/* ./Imports */

/* CLI */
#[derive(Parser)]
#[command(
    name = "PIZ",
    version,
    about = "The world's first CLI with an expansion algorithm, built to promote social responsibility.",
    long_about = "PIZ is, at its core, a social responsibility project. It aims to attract attention by becoming the world's first CLI powered by an expansion algorithm, encouraging people to raise awareness of social responsibility initiatives and charitable organizations."
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
/* ./CLI */

/* Command */
#[derive(Subcommand)]
pub enum Commands {
    // Command list
    Expand(ExpandArgs),
}

#[derive(Args, Debug)]
pub struct ExpandArgs {
    pub file: String,

    // Args list
    #[arg(long, conflicts_with = "size")]
    pub add: Option<String>,

    #[arg(long, conflicts_with = "add")]
    pub size: Option<String>,

    #[arg(long, default_value = "pi")]
    pub fill: String,

    #[arg(long, short)]
    pub output: Option<String>,
}
/* ./Command */

/* Functions */
pub fn run() {
    let cli = Cli::parse();

    if let Err(e) = crate::commands::handle_command(cli.command) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
/* ./Functions */
