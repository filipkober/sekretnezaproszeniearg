use std::io;

use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::Constraint,
    style::{Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

pub struct SelectableListOption<'a, T> {
    pub name: Spans<'a>,
    pub value: T,
}

impl<'a, T: Clone> Clone for SelectableListOption<'a, T> {
    fn clone(&self) -> Self {
        SelectableListOption {
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn render_list<T: Clone>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    box_title: &str,
    options: &Vec<SelectableListOption<T>>,
    text_before: Option<&Vec<Spans>>,
) -> Result<T, io::Error> {
    let mut state = ListState::default();
    let mut selected_id = Option::Some(0);
    state.select(selected_id);

    let text_before_width = if let Some(spans) = text_before {
        if spans.len() <= 5 {
            spans[0].width() / spans.len()
        } else {
            spans[0].width() / (spans.len() / 4)
        }
    } else {
        0
    };

    loop {
        terminal.draw(|f| {
            let block = Block::default()
                .border_type(tui::widgets::BorderType::Thick)
                .borders(Borders::all())
                .title(box_title);

            let mut paragraph: Option<Paragraph> = None;
            if let Some(spans) = text_before {
                paragraph = Some(
                    Paragraph::new(spans.clone()).block(Block::default().borders(Borders::NONE)),
                );
            }

            let chunks = tui::layout::Layout::default()
                .constraints(
                    [
                        Constraint::Length(text_before_width as u16),
                        Constraint::Max(100)
                    ]
                    .as_ref(),
                )
                .margin(1)
                .split(f.size());

            if let Some(p) = paragraph {
                f.render_widget(p, chunks[0]);
            }

            let list_items: Vec<ListItem> = options
                .iter()
                .map(|o| ListItem::new(o.name.clone()))
                .collect();
            let list = List::new(list_items)
                .highlight_symbol(">>> ")
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::NONE));
            f.render_stateful_widget(list, chunks[1], &mut state);

            f.render_widget(block, f.size())
        })?;
        if let Ok(e) = read() {
            match e {
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    code: kc,
                    ..
                }) => match kc {
                    KeyCode::Down | KeyCode::Char('s') => {
                        let mut new_id = Option::Some(0);
                        if selected_id.unwrap() < options.len() - 1 {
                            new_id = Option::Some(selected_id.unwrap() + 1);
                        }
                        selected_id = new_id;
                        state.select(selected_id);
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        let mut new_id = Option::Some(options.len() - 1);
                        if selected_id.unwrap() > 0 {
                            new_id = Option::Some(selected_id.unwrap() - 1);
                        }
                        selected_id = new_id;
                        state.select(selected_id);
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        };
    }
    Ok(if selected_id.is_some() {
        options[selected_id.unwrap()].value.clone()
    } else {
        options[0].value.clone()
    })
}

pub fn keyboard_input(buffer: &mut String, on_submit: &mut dyn FnMut(&mut String)) {
    if let Ok(event) = read() {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                buffer.push(c);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                buffer.pop();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                on_submit(buffer);
                buffer.clear();
            }
            _ => {}
        }
    }
}

pub fn exit_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), io::Error> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
