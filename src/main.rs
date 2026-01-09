mod app;
mod cli;
mod commands;
mod media;
mod render;
mod terminal;
mod util;

use clap::Parser;
use cli::args::CliArgs;

fn main() {
    let args = CliArgs::parse();
    app::launch(args);
}
