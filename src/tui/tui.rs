use crate::app::State;
use crate::prelude::*;
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct TUI {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TUI {
    pub fn get_input(&mut self, state: &State) -> Result<String> {
        let mut field = String::new();
        loop {
            self.terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(20)
                    .constraints([Constraint::Min(1), Constraint::Max(1)])
                    .split(f.area());

                let (paragraph, title, help_message) = match state {
                    State::Username => (format!("Username: {}", field), "Input Username", ""),
                    State::Room => (
                        format!("Room: {}", field),
                        "Input Room",
                        "Leave empty to create a new room.",
                    ),
                };

                let block = Paragraph::new(paragraph)
                    .block(Block::default().borders(Borders::ALL).title(title));
                f.render_widget(block, chunks[0]);

                let help_block = Paragraph::new(help_message).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );

                f.render_widget(help_block, chunks[1]);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => break,
                    KeyCode::Char(char) => {
                        if char == 'c' && key.modifiers == KeyModifiers::CONTROL {
                            return Err(Error::ProgramTerminated("CTRL-C".to_string()));
                        }
                        field.push(char);
                    }
                    KeyCode::Backspace => {
                        field.pop();
                    }
                    KeyCode::Esc => {
                        return Err(Error::ProgramTerminated("ESC".to_string()));
                    }
                    _ => {}
                }
            }
        }
        Ok(field)
    }

    pub fn display_sse(
        &mut self,
        room: String,
        message_lock: Arc<Mutex<Vec<String>>>,
    ) -> Result<()> {
        loop {
            // Fetch current messages
            let messages: Vec<String> = { message_lock.lock().unwrap().clone() };

            self.terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Min(1), Constraint::Max(1)])
                    .split(f.area());

                let block = Paragraph::new(messages.join("\n\n")).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Room {}", room)),
                );
                f.render_widget(block, chunks[0]);

                let help_block = Paragraph::new("Placeholder").style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );

                f.render_widget(help_block, chunks[1]);
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => break,
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for TUI {
    fn default() -> Self {
        let terminal = ratatui::init();
        TUI { terminal }
    }
}
