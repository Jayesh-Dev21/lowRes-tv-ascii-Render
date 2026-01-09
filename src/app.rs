use crate::cli::args::{CliArgs, Command};
use crate::commands;

pub fn launch(args: CliArgs) {
    match args.command {
        Command::Play(cfg) => commands::play::execute(cfg),
        Command::Info(cfg) => {
            if let Err(e) = commands::info::execute(cfg) {
                eprintln!("Error: {}", e);
            }
        }
        Command::Check => commands::check::execute(),
    }
}
