use std::io;

mod entropy;
mod termutils;
mod util;

use entropy::entropy::entropy;
use crate::util::save::save::{save_settings, Level, read_settings, settings_exist};
use termutils::termutils::{setup_terminal, exit_terminal};

#[allow(unused_variables)]
fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;

    if !settings_exist() {
        let (entropy, answers) = entropy(&mut terminal)?;
    }
    let mut settings = read_settings()?;

    exit_terminal(&mut terminal)
    // Ok(())
}
