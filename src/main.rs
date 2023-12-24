use std::io;

mod entropy;
mod levels;
mod util;

use crate::util::save::{read_settings, settings_exist};
use crate::util::termutils::{exit_terminal, setup_terminal};
use entropy::entropy;
use levels::mainmenu::{help, settings, level_select, reward};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use util::termutils::{render_list, SelectableListOption};

#[derive(Clone, PartialEq)]
enum MenuOption {
    Select,
    Help,
    Settings,
    Exit,
    Reward
}

#[allow(unused_variables)]
fn main() -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;

    if !settings_exist() {
        let (entropy, answers) = entropy(&mut terminal)?;
    }

    let mut render_options: Vec<SelectableListOption<MenuOption>> = vec![
        SelectableListOption {
            name: "Wybierz poziom".into(),
            value: MenuOption::Select,
        },
        SelectableListOption {
            name: "Pomoc".into(),
            value: MenuOption::Help,
        },
        SelectableListOption {
            name: "Ustawienia".into(),
            value: MenuOption::Settings,
        },
        SelectableListOption {
            name: "Wyjdź".into(),
            value: MenuOption::Exit,
        },
    ];

    let text_before = vec![
        Spans::from(vec![Span::styled(
            "          _        __            ",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![Span::styled(
            "  ___ _ _| |_ _ _ /  \\ _ __ _  _ ",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![Span::styled(
            " / -_) ' \\  _| '_| () | '_ \\ || |",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![Span::styled(
            " \\___|_||_\\__|_|  \\__/| .__/\\_, |",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![Span::styled(
            "                      |_|   |__/ ",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
    ];

    loop {
        let mut savefile = read_settings()?;

        let completed_levels = savefile
            .levels
            .iter()
            .filter(|level| level.completed)
            .count();

        if completed_levels == 18 {
            if !render_options.iter().any(|option| option.value == MenuOption::Reward) {
            render_options.push(SelectableListOption {
                name: "Nagroda".into(),
                value: MenuOption::Reward,
            })
        }
        }
        
        let selected: MenuOption =
            render_list(&mut terminal, "Menu", &render_options, Some(&text_before)).unwrap();

        match selected {
            MenuOption::Select => level_select(&mut terminal, &mut savefile).unwrap(),
            MenuOption::Help => help(&mut terminal),
            MenuOption::Settings => settings(&mut terminal, &mut savefile),
            MenuOption::Exit => break,
            MenuOption::Reward => reward(&mut terminal),
        }
    }
    exit_terminal(&mut terminal)
    // Ok(())
}
