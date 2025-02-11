use std::sync::Arc;
use std::sync::Mutex;

use crate::error;
use crate::prelude::*;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Constraint;
use ratatui::{
    DefaultTerminal, Frame,
    layout::Layout,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Running,
    Cancelled,
}

#[derive(Debug, Default)]
pub struct MessageReceiver {
    room: String,
    messages: Vec<String>,
    state: State,
}

impl MessageReceiver {
    pub fn run(
        mut self,
        terminal: &mut DefaultTerminal,
        room: String,
        message_lock: Arc<Mutex<Vec<String>>>,
    ) -> Result<()> {
        self.room = room;
        while self.state == State::Running {
            self.messages = message_lock.lock().unwrap().clone();
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }

        match self.state {
            State::Cancelled => Err(error::Error::Cancelled),
            State::Running => unreachable!(),
        }
    }

    fn render(&self, frame: &mut Frame) {
        let [area] = Layout::horizontal([Constraint::Percentage(100)])
            .margin(2)
            .areas(frame.area());

        frame.render_widget(self.widget(), area);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Esc => self.state = State::Cancelled,
                KeyCode::Char(c) => match c {
                    'q' => self.state = State::Cancelled,
                    _ => (),
                },
                _ => self.on_key_press(event),
            },
            _ => {}
        }
        Ok(())
    }

    fn on_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => {
                match c {
                    'r' => (), // !TODO: Roll
                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn widget(&self) -> Paragraph {
        Paragraph::new(self.messages.join("\n\n")).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Room {}", self.room)),
        )
    }
}
