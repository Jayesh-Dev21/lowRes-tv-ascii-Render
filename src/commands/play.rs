use crate::{
    cli::args::PlayArgs,
    media::{source, stream},
    render::{ascii, frame},
    terminal::{cursor, guard::TerminalGuard},
    util::shutdown::ShutdownFlag,
};
use pixel2ascii::AsciiOptions;

pub fn execute(cfg: PlayArgs) {
    let shutdown = ShutdownFlag::install();
    let _terminal = TerminalGuard::new();

    let video = source::resolve(&cfg).expect("Failed to resolve video");

    let mut pipeline = stream::spawn(&video, &cfg).expect("ffmpeg failed");

    let ascii_cfg = AsciiOptions {
        width: video.width as u32,
        invert: cfg.invert,
        color: !cfg.no_color,
        charset: cfg.charset,
        charset_preset: cfg.preset.into(),
        ..Default::default()
    };

    while shutdown.is_running() {
        let buffer = pipeline.read_frame().unwrap();
        let img = frame::rgb_to_image(&buffer, video.width, video.height);
        let ascii = ascii::render(&img, &ascii_cfg);
        cursor::draw(&ascii);
    }
}
