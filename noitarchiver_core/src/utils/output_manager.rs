use crate::NAResult;

pub trait OutputManager {
    fn fatal_error(&self, msg: String);
    fn warning(&self, msg: String);
    fn log(&self, msg: String);
    fn log_green(&self, msg: String);
    fn debug(&self, msg: String);
    fn getline(&self, input_msg: String) -> NAResult<String>;
    fn confirm(&self, msg: String) -> NAResult<bool>;
}
