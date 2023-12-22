use std::{
    fs::File,
    io::{self, Write},
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

pub fn level(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, savefile: &mut Savefile) {
    let bf_string = "++++++++[>+++++++<-]>+..";
    let ans = "99".to_string();

    let time = savefile.levels[0].time;
    let timestamp_0 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut input_buffer = String::new();

    let hints = vec![
        Spans::from("Podpowiedź 1: Jakiegoś rodzaju kodowanie..."),
        Spans::from("Podpowiedź 2: Być może plik powinien nazywać się 80808.bf"),
        Spans::from("Odpowiedź: 99"),
    ];

    let mut file = File::create("80808.txt").unwrap();
    file.write_all(bf_string.as_bytes()).unwrap();

    let mut endlevel = false;

    loop {
        terminal
            .draw(|f| {
                let block = tui::widgets::Block::default()
                    .title("80808")
                    .borders(tui::widgets::Borders::ALL);

                let mut text = vec![
                    Spans::from("Coś pojawiło się w plikach..."),
                    Spans::from(""),
                ];

                if savefile.levels[0].used_hints > 0 {
                    for i in 0..savefile.levels[0].used_hints {
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
            if *output == ans {
                savefile.levels[0].completed = true;
                save_savefile(savefile).unwrap();
                endlevel = true;
            } else if *output == "exit" {
                endlevel = true;
            } else if *output == "hint" && savefile.levels[0].used_hints < 3 {
                savefile.levels[0].used_hints += 1;
                save_savefile(savefile).unwrap();
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

    savefile.levels[0].time = time + (timestamp_1 - timestamp_0) as f32;

    save_savefile(savefile).unwrap();

    if savefile.levels[0].completed {
        let mut file = File::create("80808.txt").unwrap();
        file.write_all(bf_string.as_bytes()).unwrap();
        terminal
            .draw(|f| {
                let block = Block::default().title("80808").borders(Borders::ALL);
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
