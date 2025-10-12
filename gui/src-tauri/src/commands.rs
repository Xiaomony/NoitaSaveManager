use super::CORE;
use crate::gui_output::GuiOutput;
use noita_save_manager_core::{Core, NSComResult, NSResult, SingleSave, throw};
use std::sync::MutexGuard;

fn get_core<'a>() -> NSResult<MutexGuard<'a, Core<GuiOutput>>> {
    let Ok(core) = CORE.get().unwrap().try_lock() else {
        return throw("backend buzy");
    };
    Ok(core)
}

#[tauri::command]
pub fn get_saves() -> NSResult<Vec<SingleSave>> {
    Ok(get_core()?.get_save_infos().saves.clone())
}

#[tauri::command]
pub fn cmd_startgame() -> NSComResult {
    let core = get_core()?;
    core.startgame()
}

#[tauri::command]
pub fn cmd_usage() -> NSResult<f64> {
    Core::<GuiOutput>::usage_by_mb()
}

#[tauri::command]
pub fn cmd_save(name: String, note: String) -> NSComResult {
    let mut core = get_core()?;
    core.save(name, note)
}
