# LOW Resolution - YT-TV ASCII Render

<> YT-DLP

<> FFMPEG

<> RUST

<> Crate: [pixel2ascii](https://crates.io/crates/pixel2ascii)


---

Test Now using - 
```bash
./lowRes-tv-ascii-Render play "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

folder structure
```
src/
├── main.rs                 # Entry point (very thin)
├── app.rs                  # High-level orchestration
│
├── cli/
│   ├── mod.rs
│   └── args.rs             # clap definitions
│
├── terminal/
│   ├── mod.rs
│   ├── cursor.rs           # hide/show/move/clear
│   └── guard.rs            # RAII terminal cleanup
│
├── media/
│   ├── mod.rs
│   ├── source.rs           # resolve URLs, fps, metadata
│   ├── stream.rs           # spawn ffmpeg, pipe frames
│   └── audio.rs            # platform audio routing  
│
├── render/
│   ├── mod.rs
│   ├── ascii.rs            # pixel → ascii
│   └── frame.rs            # RGB buffer → image
│
├── commands/
│   ├── mod.rs
│   ├── play.rs
│   ├── info.rs
│   └── check.rs
│
└── util/
    ├── mod.rs
    └── shutdown.rs         # ctrl+c handling

```