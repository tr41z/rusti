use std::env;

mod commands;
mod probing;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set the default log level to info
    env::set_var("RUST_LOG", "info");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![init_sniffer, init_enum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn init_sniffer(target_ip: String, wordlist_path: String) {
    commands::commands::start_scanner(target_ip, wordlist_path);
}

#[tauri::command]
fn init_enum() {
    commands::commands::start_enumerator();
}