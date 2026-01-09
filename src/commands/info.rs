use crate::cli::args::InfoArgs;
use serde_json::Value;
use std::process::Command;

pub fn execute(cfg: InfoArgs) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("yt-dlp")
        .args(["-j", &cfg.url])
        .output()?;

    let json: Value = serde_json::from_slice(&output.stdout)?;

    println!("Title: {}", json["title"].as_str().unwrap_or("N/A"));
    println!("Uploader: {}", json["uploader"].as_str().unwrap_or("N/A"));
    println!("Duration: {} sec", json["duration"].as_i64().unwrap_or(0));

    Ok(())
}
