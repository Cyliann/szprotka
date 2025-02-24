// look at: https://github.com/ratatui/ratatui/blob/main/examples/apps/input-form/src/main.rs
use crate::{error, prelude::*};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout, Offset},
    widgets::{BorderType, Paragraph, Wrap},
};

use super::utils;

#[derive(Default, PartialEq, Eq)]
enum FormState {
    #[default]
    Running,
    Cancelled,
    Submitted,
}

pub struct LoginForm {
    focus: Focus,
    username: StringField,
    room: StringField,
    state: FormState,
    show_popup: bool,
}

impl LoginForm {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(String, String)> {
        while self.state == FormState::Running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        match self.state {
            FormState::Cancelled => Err(error::Error::Cancelled),
            FormState::Submitted => Ok((self.username.value, self.room.value)),
            FormState::Running => unreachable!(),
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        if self.show_popup {
            match event::read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    self.show_popup = false;
                }
                _ => (),
            }
        } else {
            match event::read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                    KeyCode::Esc => self.state = FormState::Cancelled,
                    KeyCode::Enter => {
                        self.show_popup = self.username.value == "";
                        if !self.show_popup {
                            self.state = FormState::Submitted;
                        }
                    }
                    _ => self.on_key_press(event),
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn on_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Tab => self.focus = self.focus.next(),
            _ => match self.focus {
                Focus::Username => self.username.on_key_press(event),
                Focus::Room => self.room.on_key_press(event),
            },
        }
    }

    fn render(&self, frame: &mut Frame) {
        let [area] = Layout::horizontal([Constraint::Percentage(30)])
            .flex(Flex::Center)
            .areas(frame.area());

        let [username_area, _, room_area, keybinds_area] =
            Layout::vertical(Constraint::from_lengths([3, 2, 3, 3]))
                .flex(Flex::Center)
                .areas(area);

        frame.render_widget(self.username.widget(), username_area);
        frame.render_widget(self.room.widget(), room_area);
        frame.render_widget(keybinds_widget(), keybinds_area);

        let cursor_position = match self.focus {
            Focus::Username => username_area.offset(self.username.cursor_offset()),
            Focus::Room => room_area.offset(self.room.cursor_offset()),
        };
        frame.set_cursor_position(cursor_position);

        if self.show_popup {
            show_empty_username_popup(frame);
        }
    }
}

impl Default for LoginForm {
    fn default() -> Self {
        Self {
            focus: Focus::Username,
            username: StringField::new("Username"),
            room: StringField::new("Room (leave empty to create a new room)"),
            state: FormState::default(),
            show_popup: false,
        }
    }
}

#[derive(Default, PartialEq, Eq)]
enum Focus {
    #[default]
    Username,
    Room,
}

impl Focus {
    const fn next(&self) -> Self {
        match self {
            Self::Username => Self::Room,
            Self::Room => Self::Username,
        }
    }
}

#[derive(Debug)]
struct StringField {
    label: &'static str,
    value: String,
}

impl StringField {
    const fn new(label: &'static str) -> Self {
        Self {
            label,
            value: String::new(),
        }
    }

    fn on_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => self.value.push(c),
            KeyCode::Backspace => {
                self.value.pop();
            }
            _ => {}
        }
    }

    fn cursor_offset(&self) -> Offset {
        let x = self.value.len() as i32 + 1;
        Offset { x, y: 1 }
    }

    fn widget(&self) -> Paragraph {
        Paragraph::new(self.value.to_string()).block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(BorderType::Rounded)
                .title_top(self.label),
        )
    }
}

fn keybinds_widget() -> Paragraph<'static> {
    let keybinds = vec!["Tab to switch field", "Enter to proceed", "Esc to quit"];
    Paragraph::new(keybinds.join(",  ")).wrap(Wrap { trim: true })
}

fn show_empty_username_popup(frame: &mut Frame) {
    let title = "Invalid input";
    let text = "Username cannot be empty\n\n\n[OK]";
    let dim = utils::Dimensions {
        percent_x: 20,
        percent_y: 15,
        min_x: 35,
        min_y: 5,
    };
    utils::popup(frame, dim, title, text);
}
