use std::sync::MutexGuard;

use noita_save_manager_core::{Core, NSComResult, NSResult, SingleSave, throw};

use crate::gui_output::GuiOutput;

use super::CORE;

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
pub fn cmd_save(name: String, note: String) -> NSComResult {
    let mut core = get_core()?;
    core.save(name, note)
}
