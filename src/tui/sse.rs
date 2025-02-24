use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::error;
use crate::prelude::*;
use crate::web::req;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use super::utils;

#[derive(Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Running,
    Exiting,
    Exited,
}

#[derive(Debug, Default)]
pub struct MessageReceiver {
    room: String,
    messages: Vec<String>,
    state: State,
    token: String,
}

impl MessageReceiver {
    pub async fn run(
        mut self,
        terminal: &mut DefaultTerminal,
        room: String,
        message_lock: Arc<Mutex<Vec<String>>>,
        token: String,
    ) -> Result<()> {
        self.room = room;
        self.token = token;
        while self.state == State::Running || self.state == State::Exiting {
            self.messages = message_lock.lock().unwrap().clone();
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events().await?;
        }

        match self.state {
            State::Exited => Err(error::Error::Cancelled),
            State::Exiting => unreachable!(),
            State::Running => unreachable!(),
        }
    }

    fn render(&self, frame: &mut Frame) {
        if self.state == State::Exiting {
            exit_popup(frame);
        }
        let [area] = Layout::horizontal([Constraint::Percentage(100)])
            .margin(2)
            .areas(frame.area());

        frame.render_widget(self.widget(), area);
    }

    async fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                    KeyCode::Esc => self.state = State::Exiting,
                    KeyCode::Char(c) => match c {
                        'q' => self.state = State::Exiting,
                        'r' => self.roll().await?,
                        'y' => {
                            if self.state == State::Exiting {
                                self.state = State::Exited;
                            }
                        }
                        'n' => {
                            if self.state == State::Exiting {
                                self.state = State::Running;
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn widget(&self) -> Paragraph {
        Paragraph::new(self.messages.join("\n\n")).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title_top(format!("Room {}", self.room))
                .title_bottom(get_keybinds()),
        )
    }

    async fn roll(&mut self) -> Result<()> {
        let client = reqwest::Client::new();

        let res = req::roll(&client, &self.token, 100).await;
        // TODO: handle response codes
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

fn get_keybinds() -> String {
    let keybinds = vec!["r to roll", "Esc/q to quit"];

    keybinds.join(",  ")
}

fn exit_popup(frame: &mut Frame) {
    let title = "Confirm Exit";
    let text = "Are you sure you want to exit?\n\n\n[Y] Yes   [N] No";
    let dimensions = utils::Dimensions {
        percent_x: 20,
        percent_y: 15,
        min_x: 35,
        min_y: 5,
    };
    utils::popup(frame, dimensions, title, text);
}
