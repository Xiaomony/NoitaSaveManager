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
pub fn cmd_setpath(new_path: String) -> NSComResult {
    let mut core = get_core()?;
    core.set_noita_path(new_path)
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

#[tauri::command]
pub fn cmd_qsave() -> NSComResult {
    let mut core = get_core()?;
    core.quick_save(false)
}

#[tauri::command]
pub fn cmd_overwrite() -> NSComResult {
    let mut core = get_core()?;
    core.overwrite_save()
}

#[tauri::command]
pub fn cmd_delete(indexs: Vec<usize>) -> NSComResult {
    let mut core = get_core()?;
    core.delete_saves(indexs)
}

#[tauri::command]
pub fn cmd_modify(index: usize, new_name: String, new_note: String) -> NSComResult {
    let mut core = get_core()?;

    let name_opt = if new_name.is_empty() {
        None
    } else {
        Some(new_name)
    };
    let note_opt = if new_note.is_empty() {
        None
    } else {
        Some(new_note)
    };
    core.modify_save_info(index, name_opt, note_opt)
}
