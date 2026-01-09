# LOW Resolution — YT-TV ASCII Render

A tiny, opinionated Rust tool that renders low-resolution YouTube streams as ASCII in your terminal.

Core ideas:
- Use `ffmpeg` (via a spawned process) to extract frames and audio.
- Convert frames to a small RGB buffer and map pixels to ASCII characters using the `pixel2ascii` crate.
- Keep the terminal tidy via RAII guards and cursor helpers.

Key dependencies:
- yt-dlp (or other way to provide a video URL)
- ffmpeg
- Rust toolchain (stable, edition 2024)
- Crate: `pixel2ascii` (included in `Cargo.toml`)

Quick test
```bash
./lowRes-tv-ascii-Render play "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

If you built from source with Cargo, you can also run:
```bash
cargo run -- play "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

Requirements
- ffmpeg installed and available on PATH
- yt-dlp (recommended) to resolve YouTube URLs (or provide a local file/stream)
- Rust (for building from source)

Features
- Low-resolution ASCII rendering optimized for terminal playback
- Audio routing (platform-dependent) so you can hear audio while viewing ASCII frames
- Simple CLI using `clap` with `play`, `info`, and `check` commands

Usage examples
- Play a YouTube URL (example):
    ```bash
    ./lowRes-tv-ascii-Render play "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
    ```
- Get info about a source (fps, dimensions):
    ```bash
    ./lowRes-tv-ascii-Render info <source>
    ```
- Run a quick check to ensure dependencies are available:
    ```bash
    ./lowRes-tv-ascii-Render check
    ```

Project layout
```
src/
├── main.rs                 # Entry point (very thin)
├── app.rs                  # High-level orchestration
├── cli/
│   ├── mod.rs
│   └── args.rs             # clap definitions
├── terminal/
│   ├── mod.rs
│   ├── cursor.rs           # hide/show/move/clear
│   └── guard.rs            # RAII terminal cleanup
├── media/
│   ├── mod.rs
│   ├── source.rs           # resolve URLs, fps, metadata
│   ├── stream.rs           # spawn ffmpeg, pipe frames
│   └── audio.rs            # platform audio routing
├── render/
│   ├── mod.rs
│   ├── ascii.rs            # pixel → ascii
│   └── frame.rs            # RGB buffer → image
├── commands/
│   ├── mod.rs
│   ├── play.rs
│   ├── info.rs
│   └── check.rs
└── util/
        ├── mod.rs
        └── shutdown.rs         # ctrl+c handling
```

Development
- Build in release for best performance:
    ```bash
    cargo build --release
    ./target/release/lowRes-tv-ascii-Render play <source>
    ```
- Use `cargo run -- <command>` while iterating on code.

Notes & tips
- Terminal size and font affect the appearance — smaller fonts and wider terminals look better.
- If audio doesn't play, check platform-specific code in `src/media/audio.rs` and that your system audio is configured.

Contributing
- Issues and PRs welcome. Small, focused changes are easiest to review (fixes, docs, small features).

License
- See the `LICENSE` file in the repository.

Enjoy ASCII YT in Low Res! 😄

--> Inspired by - [SameerVers3](https://github.com/SameerVers3/stdout-tv)
