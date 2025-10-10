// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Manager,
    window::{Effect, EffectsBuilder},
};

fn main() {
    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
