pub mod expand;

use crate::cli::Commands;

pub fn handle_command(cmd: Commands) -> Result<(), String> {
    match cmd {
        Commands::Expand(args) => expand::exec(args),
    }
}
