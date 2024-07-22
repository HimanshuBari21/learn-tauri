// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn num_add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, num_add])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![num_add])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
