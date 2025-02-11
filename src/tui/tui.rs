use crate::{error, prelude::*};
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode};
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
    pub fn display_sse(
        &mut self,
        room: &String,
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
                        KeyCode::Esc => return Err(error::Error::Cancelled),
                        KeyCode::Char(c) => {
                            if c == 'q' {
                                return Err(error::Error::Cancelled);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

impl Default for TUI {
    fn default() -> Self {
        let terminal = ratatui::init();
        TUI { terminal }
    }
}
