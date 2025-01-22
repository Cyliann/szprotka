use crate::error::Error;
use crate::web;
use std::{io, thread::sleep, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct App {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    state: CurrentField,
    pub user: User,
}

pub struct User {
    pub username: String,
    pub room: String,
    token: String,
}

enum CurrentField {
    Username,
    Room,
}

impl App {
    pub fn get_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut username = String::new();
        let mut room = String::new();
        self.state = CurrentField::Username;
        loop {
            self.terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(20)
                    .constraints([Constraint::Min(1), Constraint::Max(1)])
                    .split(f.area());

                let block;
                match self.state {
                    CurrentField::Username => {
                        block = Paragraph::new(format!("Username: {}", username)).block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title("Enter a username"),
                        );
                    }
                    CurrentField::Room => {
                        block = Paragraph::new(format!("Room: {}", room))
                            .block(Block::default().borders(Borders::ALL).title("Input Room"));
                    }
                }
                f.render_widget(block, chunks[0]);

                let help_message = match self.state {
                    CurrentField::Username => "",
                    CurrentField::Room => "Leave empty to create a new room.",
                };

                let help_block = Paragraph::new(help_message).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );

                f.render_widget(help_block, chunks[1]);
            })?;
            if let Event::Key(key) = event::read()? {
                let field: &mut String;
                match self.state {
                    CurrentField::Username => field = &mut username,
                    CurrentField::Room => field = &mut room,
                }

                match key.code {
                    KeyCode::Enter => match self.state {
                        CurrentField::Username => self.state = CurrentField::Room,
                        CurrentField::Room => break,
                    },
                    KeyCode::Char(char) => {
                        if char == 'c' && key.modifiers == KeyModifiers::CONTROL {
                            _ = self.close(Some(Error::ProgramTerminated("CTRL-C".to_string())));
                        }
                        field.push(char);
                    }
                    KeyCode::Backspace => {
                        field.pop();
                    }
                    KeyCode::Esc => {
                        _ = self.close(Some(Error::ProgramTerminated("ESC".to_string())));
                    }
                    _ => {}
                }
            }
        }
        self.user.username = username;
        self.user.room = room;
        Ok(())
    }

    pub async fn subscribe(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let (token, room) =
            web::req::register(&client, &self.user.username, &self.user.room).await?;
        self.user.room = room;
        self.user.token = token;
        println!("Room: {}", &self.user.room);

        let token_clone = self.user.token.clone();
        tokio::task::spawn(async move { web::sse::handle_sse(token_clone).await });

        loop {
            if false {
                break;
            }
            web::req::roll(&client, &self.user.token).await?;
            sleep(Duration::from_secs(3));
        }
        Ok(())
    }

    pub fn close(&mut self, err: Option<Error>) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            event::PopKeyboardEnhancementFlags
        )?;
        self.terminal.show_cursor()?;
        if let Some(err) = err {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Ok(())
    }
}

pub fn new() -> Result<App, Box<dyn std::error::Error>> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        event::PushKeyboardEnhancementFlags(
            event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        ),
        event::PushKeyboardEnhancementFlags(
            event::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )
    )?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(App {
        terminal,
        state: CurrentField::Username,
        user: User {
            username: String::new(),
            room: String::new(),
            token: String::new(),
        },
    })
}
