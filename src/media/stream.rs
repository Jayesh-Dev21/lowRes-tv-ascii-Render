use super::source::VideoSource;
use crate::cli::args::PlayArgs;
use crate::media::audio;
use std::{
    io::Read,
    process::{Child, Command, Stdio},
};

pub struct MediaStream {
    child: Child,
    frame_size: usize,
}

pub fn spawn(
    video: &VideoSource,
    cfg: &PlayArgs,
) -> Result<MediaStream, Box<dyn std::error::Error>> {

    let mut cmd = Command::new("ffmpeg");

    // Input 
    cmd.args([
        "-fflags", "nobuffer",
        "-flags", "low_delay",
        "-i", &video.url,
    ]);

    // Video output (pipe → Rust)
    cmd.args([
        "-map", "0:v:0",
        "-vf", &format!("scale={}:{}", video.width, video.height),
        "-pix_fmt", "rgb24",
        "-f", "rawvideo",
        "pipe:1",
    ]);

    // Audio output (direct to system)
    if !cfg.no_audio {
        let (audio_fmt, audio_dev) = audio::output_device();

        cmd.args([
            "-map", "0:a:0",
            "-f", audio_fmt,
            "-ac", "2",
            "-ar", "44100",
            audio_dev,
        ]);
    }

    cmd.stdout(Stdio::piped())
        .stderr(Stdio::null());

    let child = cmd.spawn()?;

    Ok(MediaStream {
        child,
        frame_size: (video.width * video.height * 3) as usize,
    })
}

impl MediaStream {
    pub fn read_frame(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = vec![0u8; self.frame_size];

        let stdout = self
            .child
            .stdout
            .as_mut()
            .expect("ffmpeg stdout unavailable");

        stdout.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}
