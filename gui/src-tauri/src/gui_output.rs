use super::APP_HANDLE;
use noita_save_manager_core::output_manager::OutputManager;
use serde::Serialize;
use tauri::Emitter;

#[derive(Debug)]
pub struct GuiOutput {}

#[derive(Clone, Serialize)]
struct OutputPayload {
    log_grade: u32,
    message: String,
}

impl OutputPayload {
    pub fn new(grade: u32, msg: String) -> Self {
        Self {
            log_grade: grade,
            message: msg,
        }
    }
}

impl GuiOutput {
    fn emit_message(grade: u32, msg: String) {
        APP_HANDLE
            .get()
            .unwrap()
            .emit("warning", OutputPayload::new(grade, msg))
            .unwrap();
    }
}

impl OutputManager for GuiOutput {
    fn fatal_error(&self, msg: String) {
        GuiOutput::emit_message(1, msg);
    }

    fn warning(&self, msg: String) {
        GuiOutput::emit_message(2, msg);
    }

    fn log(&self, msg: String) {
        GuiOutput::emit_message(3, msg);
    }

    fn log_green(&self, msg: String) {
        GuiOutput::emit_message(4, msg);
    }

    fn debug(&self, msg: String) {
        GuiOutput::emit_message(5, msg);
    }

    fn confirm(&self, _msg: String) -> noita_save_manager_core::NSResult<bool> {
        todo!()
    }
}
