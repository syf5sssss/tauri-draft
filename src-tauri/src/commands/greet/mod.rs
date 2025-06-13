// src/commands/greet/mod.rs
use tauri::command;
use log::{ info, trace, warn };

#[command]
pub fn greet(name: &str) -> String {
    trace!("Hello, {}! You've been greeted from Rust!", name);
    trace!("Commencing yak shaving");
    info!("Razor located: {name}");
    warn!("Unable to locate a razor: {name}, retrying");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn log_from_js(level: String, message: String) {
    match level.as_str() {
        "info" => log::info!("[JS] {}", message),
        "warn" => log::warn!("[JS] {}", message),
        "debug" => log::debug!("[JS] {}", message),
        _ => log::error!("[JS] {}", message),
    }
}
