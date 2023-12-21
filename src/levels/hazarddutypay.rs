use std::{
    io,
    thread,
    time::Duration,
};

use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crate::util::{
    save::{save_savefile, Savefile},
    termutils::keyboard_input,
};

const LEVEL_NAME: &str = "Hazard Duty Pay!";
const LEVEL_NUM: usize = 1;
const ANS: &str = "05824935784";
const HINT1: &str = "przydałoby się wejść w ten link...";
const HINT2: &str = "może by tak zapisać to zdjęcie?";

pub fn level(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, savefile: &mut Savefile) {

    let mut input_buffer = String::new();

    let hints = vec![
        Spans::from(format!("Podpowiedź 1: {}", HINT1)),
        Spans::from(format!("Podpowiedź 2: {}", HINT2)),
        Spans::from(format!("Odpowiedź: {}", ANS)),
    ];

    let mut endlevel = false;

    loop {
        terminal
            .draw(|f| {
                let block = tui::widgets::Block::default()
                    .title(LEVEL_NAME)
                    .borders(tui::widgets::Borders::ALL);

                let mut text = vec![
                    Spans::from("https://entr0py.nigdit.me/hdp"),
                    Spans::from(""),
                ];

                if savefile.levels[LEVEL_NUM].used_hints > 0 {
                    for i in 0..savefile.levels[LEVEL_NUM].used_hints {
                        if let Some(hint) = hints.get(i) {
                            text.push(hint.clone())
                        }
                    }
                }
                text.push("".into());
                text.push(Spans::from(Span::styled(
                    "[dostępne komendy: hint, exit, <odpowiedź>]",
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
                text.push("".into());

                text.push(Spans::from(vec![
                    Span::styled(
                        "> ",
                        Style::default()
                            .fg(tui::style::Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::from(input_buffer.clone()),
                ]));

                let paragraph = Paragraph::new(text).block(block);

                f.render_widget(paragraph, f.size());
            })
            .unwrap();

        keyboard_input(&mut input_buffer, &mut |output| {
            if *output == ANS {
                savefile.levels[LEVEL_NUM].completed = true;
                save_savefile(savefile).unwrap();
                endlevel = true;
            } else if *output == "exit" {
                endlevel = true;
            } else if *output == "hint" && savefile.levels[LEVEL_NUM].used_hints < 3 {
                savefile.levels[LEVEL_NUM].used_hints += 1;
                save_savefile(savefile).unwrap();
            }
        });

        if endlevel {
            break;
        }
    }

    if savefile.levels[LEVEL_NUM].completed {
        terminal
            .draw(|f| {
                let block = Block::default().title(LEVEL_NAME).borders(Borders::ALL);
                let para = Paragraph::new(Span::styled(
                    "Poprawna odpowiedź, wychodzenie za 3 sekundy...",
                    Style::default().fg(tui::style::Color::Green),
                ))
                .block(block);

                f.render_widget(para, f.size());
            })
            .unwrap();

        thread::sleep(Duration::from_secs(3));
    }
}
