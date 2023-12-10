use std::io;

mod entropy;
mod util;

use entropy::entropy::entropy;
use crate::util::save::save::{read_settings, settings_exist};
use crate::util::termutils::termutils::{setup_terminal, exit_terminal};

#[allow(unused_variables)]
fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;

    if !settings_exist() {
        let (entropy, answers) = entropy(&mut terminal)?;
    }
    let settings = read_settings()?;

    exit_terminal(&mut terminal)
    // Ok(())
}
