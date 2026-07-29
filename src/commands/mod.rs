pub mod expand;

use crate::cli::Commands;
use crate::error::Result;

pub fn handle_command(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Expand(args) => expand::exec(args),
    }
}
