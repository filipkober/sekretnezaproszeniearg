pub mod entropy {
use std::{thread, time::Duration, io};

use base64::{engine::general_purpose, Engine};
use crossterm::event::{read, KeyEvent, Event, KeyCode, KeyEventKind};
use tui::{Terminal, widgets::{Block, Borders, Paragraph}, text::{Span, Spans}, style::{Modifier, Style}, backend::CrosstermBackend};

use crate::util::save::save::{save_settings, empty_savefile};

pub static QUESTIONS: [&str; 6] = [
    "Ulubiony kolor: ",
    "Ulubione zwierze: ",
    "Ulubiony kryzys gospodarczy: ",
    "Twoja nazwa: ",
    "Jeśli miałbyś zostać martwym obiektem, to czym byś został: ",
    "Twój najgłębszy sekret: ",
];

pub fn entropy(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(String, Vec<String>), io::Error> {
    let mut input_text = String::new();
    let mut answers: Vec<String> = vec![];

    loop {
        let block = Block::default()
            .title("Generator entropii")
            .borders(Borders::ALL);
        if answers.len() <= 5 {
            let question = match answers.len() {
                n @ 0..=6 => QUESTIONS[n],
                _ => ""
            };
            let mut input_spans = vec![
                Spans::from(vec![Span::styled(
                    "Nie wykryto entropii, proszę o udzielenie odpowiedzi...",
                    Style::default()
                        .fg(tui::style::Color::Green)
                        .add_modifier(Modifier::UNDERLINED),
                )]),
            ];
            let mut already_answered: Vec<Spans> = answers.clone().into_iter().enumerate().map(|(index, a)| {
                Spans::from(vec![
                    Span::styled(QUESTIONS[index], Style::default().fg(tui::style::Color::Gray)),
                    Span::raw(a)
                ])
            }).collect();
            input_spans.append(&mut already_answered);
            input_spans.push(Spans::from(vec![Span::raw(question), Span::raw(input_text.clone())]));
            let paragraph = Paragraph::new(input_spans).block(block);
            terminal.draw(|f| {
                let size = f.size();
                f.render_widget(paragraph, size);
            })?;
            if let Ok(event) = read() {
                match event {
                    Event::Key(KeyEvent {
                        kind: KeyEventKind::Press,
                        code,
                        ..
                    }) => match code {
                        KeyCode::Enter => {
                            answers.push(input_text.clone());
                            input_text.clear();
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Backspace => {
                            input_text.pop();
                        }
                        KeyCode::Char(c) => input_text.push(c),

                        _ => {}
                    },
                    _ => {}
                }
            }
        } else {
            break;
        }
    }

    let block = Block::default()
            .title("Generator entropii")
            .borders(Borders::ALL);
    let para = Paragraph::new("Generowanie entropii...").block(block);
    terminal.draw(|f| {
        f.render_widget(para, f.size());
    })?;

    let str_encoded: String = general_purpose::STANDARD_NO_PAD.encode(answers.clone().join(""));
    thread::sleep(Duration::from_millis(2_000));

    let block = Block::default()
            .title("Generator entropii")
            .borders(Borders::ALL);
    let mut spans: Vec<Spans> = vec![
        Spans::from("Entropia wygenerowana pomyślnie!"),
    ];
    let para = Paragraph::new(spans.clone()).block(block);
    terminal.draw(|f| {
        f.render_widget(para, f.size());
    })?;

    thread::sleep(Duration::from_millis(1_000));

    spans.push(Spans::from(vec![Span::from("Entropia: "), Span::styled(str_encoded.clone() , Style::default().fg(tui::style::Color::LightGreen))]));

    terminal.draw(|f| {
        let block = Block::default()
            .title("Generator entropii")
            .borders(Borders::ALL);
        let para = Paragraph::new(spans.clone()).block(block);
        f.render_widget(para, f.size());
    })?;

    thread::sleep(Duration::from_millis(1_000));

    spans.push(Spans::from(Span::styled("Powrót do aplikacji za 3 sekundy...", Style::default().fg(tui::style::Color::LightRed))));

    terminal.draw(|f| {
        let block = Block::default()
            .title("Generator entropii")
            .borders(Borders::ALL);
        let para = Paragraph::new(spans.clone()).block(block);
        f.render_widget(para, f.size());
    })?;

    let empty_save = empty_savefile(str_encoded.clone());

    save_settings(str_encoded.clone(), empty_save.levels)?;

    thread::sleep(Duration::from_millis(3_000));

    Ok((str_encoded, answers))
}
}