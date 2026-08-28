// Suppress the console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Thin webview shell: window/URL config lives entirely in tauri.conf.json.
// No custom commands — this app just opens the prod CRM URL in a native window.
fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running BTC CRM shell");
}
