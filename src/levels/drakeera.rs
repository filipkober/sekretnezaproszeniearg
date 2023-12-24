use std::{collections::HashMap, io, thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crate::util::{
    save::{save_savefile, Savefile},
    termutils::keyboard_input, analyticsrequests,
};

const LEVEL_NAME: &str = "Drake Era";
const LEVEL_NUM: usize = 9;
const ANS: &str = "888553";
const HINT1: &str = "nowa komenda!";
const HINT2: &str = "podałeś twój email?";

pub fn level(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, savefile: &mut Savefile) {
    let time = savefile.levels[LEVEL_NUM].time;
    let timestamp_0 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

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

                let mut text = vec![Spans::from("..."), Spans::from("")];

                if savefile.levels[LEVEL_NUM].used_hints > 0 {
                    for i in 0..savefile.levels[LEVEL_NUM].used_hints {
                        if let Some(hint) = hints.get(i) {
                            text.push(hint.clone())
                        }
                    }
                }
                text.push("".into());
                text.push(Spans::from(Span::styled(
                    "[dostępne komendy: hint, exit, email <twoj_email>, <odpowiedź>]",
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
            } else {
                let split = output.split(" ").collect::<Vec<&str>>();
                if split.len() == 2 && split[0] == "email" {
                    let email = split[1].to_owned();
                    thread::spawn(move || {
                        let mut map = HashMap::new();
                        map.insert("email", email);
                        let client = reqwest::blocking::Client::new();
                        client
                            .post("https://entr0py.nigdit.men/drakera")
                            .json(&map)
                            .send()
                            .unwrap();
                    });
                }
            }
        });

        if endlevel {
            break;
        }
    }

    let timestamp_1 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    savefile.levels[LEVEL_NUM].time = time + (timestamp_1 - timestamp_0) as f32;

    save_savefile(savefile).unwrap();

    analyticsrequests::update_level(
        savefile.entropy.clone(),
        savefile.levels[LEVEL_NUM].time.clone(),
        savefile.levels[LEVEL_NUM].used_hints.clone(),
        savefile.levels[LEVEL_NUM].completed.clone(),
        LEVEL_NUM,
    );

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
