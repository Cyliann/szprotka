use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::error;
use crate::prelude::*;
use crate::web::req;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

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
            self.exit_popup(frame);
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
                .title(format!("Room {}", self.room)),
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

    fn exit_popup(&self, frame: &mut Frame) {
        let size = frame.area();

        // Define a centered popup size
        let popup_area = centered_rect(20, 15, size);

        // Create the popup widget
        let block = Block::default()
            .title("Confirm Exit")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let text = Paragraph::new("Are you sure you want to exit?\n\n[Y] Yes   [N] No")
            .block(block)
            .alignment(Alignment::Center);

        // Render the popup
        frame.render_widget(text, popup_area);
    }
}

// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2), // Top padding
            Constraint::Percentage(percent_y),             // Popup height
            Constraint::Percentage((100 - percent_y) / 2), // Bottom padding
        ])
        .split(r);

    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2), // Left padding
            Constraint::Percentage(percent_x),             // Popup width
            Constraint::Percentage((100 - percent_x) / 2), // Right padding
        ])
        .split(popup_layout[1]);

    popup_area[1] // The centered area
}
