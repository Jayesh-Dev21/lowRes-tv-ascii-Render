pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> Self {
        print!("\x1B[2J\x1B[?25l");
        Self
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        print!("\x1B[?25h");
    }
}
