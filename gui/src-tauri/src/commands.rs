use super::{APP_HANDLE, CORE};
use crate::gui_output::GuiOutput;
use noita_save_manager_core::{Core, NSComResult, NSError, NSResult, SingleSave, throw};
use std::sync::MutexGuard;
use tauri::Emitter;

fn release_backend_lock() {
    APP_HANDLE
        .get()
        .unwrap()
        .emit("release_backend_lock", "")
        .unwrap();
}

fn get_core<'a>() -> NSResult<MutexGuard<'a, Core<GuiOutput>>> {
    let Ok(core) = CORE
        .get()
        .ok_or(NSError::new("Fail to get mutex lock"))?
        .try_lock()
    else {
        return throw("backend buzy");
    };
    Ok(core)
}

#[tauri::command]
pub fn get_saves() -> NSResult<Vec<SingleSave>> {
    let saves = get_core()?.get_save_infos().saves.clone();
    release_backend_lock();
    Ok(saves)
}

#[tauri::command]
pub fn cmd_startgame() -> NSComResult {
    let core = get_core()?;
    core.startgame()?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_setpath(new_path: String) -> NSComResult {
    let mut core = get_core()?;
    core.set_noita_path(new_path)?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_usage() -> NSResult<f64> {
    let usage = Core::<GuiOutput>::usage_by_mb()?;
    release_backend_lock();
    Ok(usage)
}

#[tauri::command]
pub fn cmd_save(name: String, note: String) -> NSComResult {
    let mut core = get_core()?;
    core.save(name, note)?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_qsave() -> NSComResult {
    let mut core = get_core()?;
    core.quick_save(false)?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_overwrite() -> NSComResult {
    let mut core = get_core()?;
    core.overwrite_save()?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_load(index: usize) -> NSComResult {
    let core = get_core()?;
    core.load_save(index)?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_qload() -> NSComResult {
    let core = get_core()?;
    core.quick_load()?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_delete(indexs: Vec<usize>) -> NSComResult {
    let mut core = get_core()?;
    core.delete_saves(indexs)?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_qdelete() -> NSComResult {
    let mut core = get_core()?;
    core.quick_delete_save()?;
    release_backend_lock();
    Ok(())
}

#[tauri::command]
pub fn cmd_modify_lock(indexs: Vec<usize>, operate: bool) -> NSComResult {
    let mut core = get_core()?;
    if operate {
        core.lock(indexs)?;
    } else {
        core.unlock(indexs)?;
    }
    release_backend_lock();
    Ok(())
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
    core.modify_save_info(index, name_opt, note_opt)?;
    release_backend_lock();
    Ok(())
}
