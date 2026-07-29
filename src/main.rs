/* Modules */
mod cli;
mod commands;
mod core;
mod error;
/* ./Modules */

fn main() -> std::process::ExitCode {
    cli::run()
}
