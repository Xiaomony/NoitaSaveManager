// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Mutex, OnceLock};
use tauri::{
    Manager,
    window::{Effect, EffectsBuilder},
};

mod commands;
mod gui_output;
use commands::*;
use gui_output::GuiOutput;
use noita_save_manager_core::Core;

pub static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
pub static CORE: OnceLock<Mutex<Core<GuiOutput>>> = OnceLock::new();
pub const GUIOPT: GuiOutput = GuiOutput {};

fn main() {
    CORE.set(Mutex::new(
        noita_save_manager_core::Core::new(GUIOPT).unwrap(),
    ))
    .unwrap();

    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    tauri::Builder::default()
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            let window = app.get_webview_window("main").unwrap();
            window.set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_saves,
            cmd_startgame,
            cmd_usage,
            cmd_save,
        ])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while starting application");
}
