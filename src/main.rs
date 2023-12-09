use std::io;

use entropy::entropy::entropy;
use save::save::{save_settings, Level, read_settings, settings_exist};
use termutils::termutils::{setup_terminal, exit_terminal};

mod entropy;
mod termutils;
mod save;

#[allow(unused_variables)]
fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;

    if(!settings_exist()){
        let (entropy, answers) = entropy(&mut terminal)?;
    } else {
    let settings = read_settings()?;
    println!("{:?}", settings);
    }
    exit_terminal(&mut terminal)
    // Ok(())
}
