use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub struct ShutdownFlag {
    running: Arc<AtomicBool>,
}

impl ShutdownFlag {
    pub fn install() -> Self {
        let flag = Arc::new(AtomicBool::new(true));
        let f = flag.clone();

        ctrlc::set_handler(move || {
            f.store(false, Ordering::SeqCst);
        }).unwrap();

        Self { running: flag }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}
