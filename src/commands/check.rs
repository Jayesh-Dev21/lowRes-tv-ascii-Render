use std::process::Command;

pub fn execute() {
    check("yt-dlp");
    check("ffmpeg");
}

fn check(bin: &str) {
    if Command::new(bin).arg("--version").output().is_ok() {
        println!("✓ {} found", bin);
    } else {
        println!("✗ {} missing", bin);
    }
}
