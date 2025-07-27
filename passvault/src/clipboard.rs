use arboard::Clipboard;
use std::{thread, time::Duration};
pub fn copy_and_clear(s: &str, timeout_secs: u64) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(s.to_string()).unwrap();
    let s = s.to_string();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(timeout_secs));
        let mut clipboard = Clipboard::new().unwrap();
        let _ = clipboard.set_text("");
    });
} 