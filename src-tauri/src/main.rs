// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::{thread, time::Duration};

#[tauri::command]
fn screenshot() -> String {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

    loop {
        // Wait until there's a frame.

        match capturer.frame() {
            Ok(_) => {
                return "Captured!".to_string();
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    return format!("Error: {}", error);
                }
            }
        };
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![screenshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
