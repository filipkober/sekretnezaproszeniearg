use std::io;

use crossterm::event::{self, read, KeyEvent};
use tui::{Terminal, backend::CrosstermBackend, text::{Spans, Span}, style::Style};

use crate::util::{save::{Savefile, empty_savefile, save_settings}, termutils::{render_list, SelectableListOption}};

use super::{levels::{LevelName, num_to_level_enum}, eightoeightoeight, hazarddutypay, krystle, rainbowsix, hollywoodbaby, westernunion, toothless, godlovesyou, knownforit, drakeera, outbysixteen, thefear, tantor, deathcamp, burfict, thetwentysevenclub, freethefrail, today};

pub fn help(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {

    let text: Vec<Spans> = vec![
        Spans::from(vec![
            Span::styled("Witaj w grze ", tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD)),
            Span::styled("Entr0py", tui::style::Style::default().fg(tui::style::Color::Green).add_modifier(tui::style::Modifier::BOLD)),
            Span::styled("!\n", tui::style::Style::default().add_modifier(tui::style::Modifier::BOLD)),
        ]),
        Spans::from(""),
        Spans::from("Gra polega na przejściu 18 poziomów"),
        Spans::from("Poziomy można przejść w dowolnej kolejności"),
        Spans::from("Aby przejść poziom musisz odpowiedzieć na pytanie"),
        Spans::from("Odpowiedzi zawsze będą w formie liczby"),
        Spans::from(""),
        Spans::from("Podpowiedzi:"),
        Spans::from("Jeśli nie wiesz jak odpowiedzieć na pytanie, w pole odpowiedzi wpisz 'hint'"),
        Spans::from("Każdy poziom ma 3 podpowiedzi, z czego trzecia jest odpowiedzią"),
        Spans::from("Wyjście do menu:"),
        Spans::from("Aby wyjść do menu, w pole odpowiedzi wpisz 'exit'"),
        Spans::from(""),
        Spans::from("Powodzenia!"),
        Spans::from(""),
        Spans::from("[Naciśnij dowolny klawisz aby wrócić do menu]"),
    ];

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = tui::widgets::Block::default()
                .title("Pomoc")
                .borders(tui::widgets::Borders::ALL);
            let paragraph = tui::widgets::Paragraph::new(text.clone()).block(block);
            f.render_widget(paragraph, size);
        }).unwrap();
        if let Ok(event) = read() {
            match event {
                event::Event::Key(
                    KeyEvent {
                        kind: event::KeyEventKind::Press,
                        ..
                    }
                ) => break,
                _ => {}
                
            }
        }
    }

}

pub fn settings(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, savefile: &mut Savefile){

    let options: Vec<SelectableListOption<String>> = vec![
        SelectableListOption {
            name: Span::styled("Zresetuj postęp", Style::default().fg(tui::style::Color::Red)).into(),
            value: "reset".to_string(),
        },
        SelectableListOption {
            name: "Wyjdź".into(),
            value: "exit".to_string(),
        },
    ];

    let chosen = render_list(terminal, "Ustawienia", &options, None).unwrap();
    match chosen.as_str() {
        "reset" => {
            let empty_savefile = empty_savefile(savefile.entropy.clone());
            save_settings(empty_savefile.entropy, empty_savefile.levels).unwrap();
        },
        "exit" => {},
        _ => unreachable!(),
    }
}

pub fn level_select(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, savefile: &mut Savefile) -> Result<(), io::Error> {

    let mut options: Vec<SelectableListOption<Option<LevelName>>> = vec![];
    for (i, level) in savefile.levels.iter().enumerate() {
        let mut style = Style::default().fg(tui::style::Color::Gray);
        if level.completed {
            style = style.fg(tui::style::Color::Green);
        }
        options.push(SelectableListOption {
            name: Span::styled(i.to_string() + ". " + level.name.clone().as_str(), style).into(),
            value: Some(num_to_level_enum(i).unwrap()),
        });
    }
    options.push(SelectableListOption {
        name: "[Wyjdź]".into(),
        value: None,
    });

    let text_before = vec![
        Spans::from("                     .::."),
        Spans::from("                  .:'  .:"),
        Spans::from("        ,MMM8&&&.:'   .:'"),
        Spans::from("       MMMMM88&&&&  .:'"),
        Spans::from("      MMMMM88&&&&&&:'"),
        Spans::from("      MMMMM88&&&&&&"),
        Spans::from("    .:MMMMM88&&&&&&"),
        Spans::from("  .:'  MMMMM88&&&&"),
        Spans::from(".:'   .:'MMM8&&&'"),
        Spans::from(":'  .:'"),
        Spans::from("'::'  ")
    ];

    let chosen = render_list(terminal, "Wybierz poziom", &options, Some(&text_before)).unwrap();
    if let Some(chosen_level) = chosen {
        match chosen_level {
            LevelName::EightOEightOEight => eightoeightoeight::level(terminal, savefile),
            LevelName::HazardDutyPay => hazarddutypay::level(terminal, savefile),
            LevelName::Krystle => krystle::level(terminal, savefile),
            LevelName::RainbowSix => rainbowsix::level(terminal, savefile),
            LevelName::HollywoodBaby => hollywoodbaby::level(terminal, savefile),
            LevelName::WesternUnion => westernunion::level(terminal, savefile),
            LevelName::Toothless => toothless::level(terminal, savefile),
            LevelName::GodLovesYou => godlovesyou::level(terminal, savefile),
            LevelName::KnownForIt => knownforit::level(terminal, savefile),
            LevelName::DrakeEra => drakeera::level(terminal, savefile),
            LevelName::OutBy16DeadOnTheScene => outbysixteen::level(terminal, savefile),
            LevelName::TheFear => thefear::level(terminal, savefile),
            LevelName::Tantor => tantor::level(terminal, savefile),
            LevelName::Deathcamp => deathcamp::level(terminal, savefile),
            LevelName::Burfict => burfict::level(terminal, savefile),
            LevelName::The27Club => thetwentysevenclub::level(terminal, savefile),
            LevelName::FreeTheFrail => freethefrail::level(terminal, savefile),
            LevelName::Today => today::level(terminal, savefile),
        }
    }

    Ok(())

}