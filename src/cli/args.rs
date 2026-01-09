use clap::{Parser, Subcommand, ValueEnum};
use pixel2ascii::convert::CharsetPreset as LibPreset;

#[derive(Parser)]
#[command(name = "lowResAscii")]
#[command(about = "Play online yt videos as ASCII in terminal")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Play(PlayArgs),
    Info(InfoArgs),
    Check,
}

#[derive(Parser)]
pub struct PlayArgs {
    pub url: String,

    #[arg(long)]
    pub width: Option<u16>,

    #[arg(long)]
    pub height: Option<u16>,

    #[arg(long)]
    pub fps: Option<u8>,

    #[arg(long)]
    pub charset: Option<String>,

    #[arg(long, value_enum, default_value_t = CharsetPreset::Default)]
    pub preset: CharsetPreset,

    #[arg(long)]
    pub invert: bool,

    #[arg(long)]
    pub no_color: bool,

    #[arg(long)]
    pub no_audio: bool,
}

#[derive(Parser)]
pub struct InfoArgs {
    pub url: String,
}

#[derive(Copy, Clone, ValueEnum)]
pub enum CharsetPreset {
    Default,
    Dense,
    Blocks,
}

impl From<CharsetPreset> for LibPreset {
    fn from(v: CharsetPreset) -> Self {
        match v {
            CharsetPreset::Default => LibPreset::Default,
            CharsetPreset::Dense => LibPreset::Dense,
            CharsetPreset::Blocks => LibPreset::Blocks,
        }
    }
}
