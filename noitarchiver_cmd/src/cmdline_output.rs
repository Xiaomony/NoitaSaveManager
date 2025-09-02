use noitarchiver_core::output_manager;

pub struct CmdlineOutput {}

impl output_manager::OutputManager for CmdlineOutput {
    fn fatal_error(&self, msg: String) {
        todo!()
    }

    fn warning(&self, msg: String) {
        println!("[WARNING] {msg}")
    }

    fn log(&self, msg: String) {
        todo!()
    }

    fn log_green(&self, msg: String) {
        todo!()
    }

    fn debug(&self, msg: String) {
        todo!()
    }
}
