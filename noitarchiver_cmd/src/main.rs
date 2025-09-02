mod cmdline_output;
use std::{thread::sleep, time::Duration};

use cmdline_output::CmdlineOutput;

use noitarchiver_core::Core;

fn main() {
    let core = Core::new(CmdlineOutput{}).unwrap();
    if let Err(e) = core.startgame() {
        println!("{e}");
    }
    sleep(Duration::from_secs(2));

}
