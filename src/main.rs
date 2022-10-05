use arboard::Clipboard;
use enigo::*;
use std::{process, thread::sleep, time::Duration};

fn main() {
    let mut enigo = Enigo::new();
    let mut clip = match Clipboard::new() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Access to clipboard denied");
            process::exit(1)
        }
    };
    let pass = match clip.get_text() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Clipboard doesn't contain text");
            process::exit(1)
        }
    };
    println!("Typing password from clipboard in 3s...");
    sleep(Duration::from_secs(3));
    if let Err(_) = enigo.key_sequence_parse_try(&pass) {
        eprintln!("Failed to send keystrokes to system");
        process::exit(1);
    }
    if let Err(_) = clip.clear() {
        eprintln!("Failed to clear the clipboard.");
        process::exit(1);
    }
    println!("Clipboard cleared");
}
