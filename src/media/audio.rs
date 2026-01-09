/// ffmpeg handles audio playback directly.
pub fn output_device() -> (&'static str, &'static str) {
    #[cfg(target_os = "linux")]
    {
        // PulseAudio / PipeWire
        ("pulse", "default")
    }

    #[cfg(target_os = "macos")]
    {
        // CoreAudio
        ("audiotoolbox", "default")
    }

    #[cfg(target_os = "windows")]
    {
        // DirectSound
        ("dsound", "default")
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        // Fallback
        ("sdl", "default")
    }
}
