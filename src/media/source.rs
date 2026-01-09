use crate::cli::args::PlayArgs;
use std::process::Command;

pub struct VideoSource {
    pub url: String,
    pub fps: f32,
    pub width: u16,
    pub height: u16,
}

pub fn resolve(cfg: &PlayArgs) -> Result<VideoSource, Box<dyn std::error::Error>> {
    let fps = cfg.fps.map(|f| f as f32).unwrap_or(24.0);

    let output = Command::new("yt-dlp")
        .args(["-f", "best", "-g", &cfg.url])
        .output()?;

    let stream_url = String::from_utf8(output.stdout)?.trim().to_string();

    let width = cfg.width.unwrap_or(80);
    let height = cfg.height.unwrap_or(45);

    Ok(VideoSource {
        url: stream_url,
        fps,
        width,
        height,
    })
}
