use crate::prelude::*;
use crate::web;
use crate::{error::Error, tui};
use std::{io, thread::sleep, time::Duration};

use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

pub struct App {
    tui: tui::TUI,
    state: State,
    pub user: User,
}

pub struct User {
    pub username: String,
    pub room: String,
    token: String,
}

pub enum State {
    Username,
    Room,
}

impl App {
    pub fn get_input(&mut self) -> Result<()> {
        self.user.username = self.tui.get_input(&self.state)?;
        self.state = State::Room;
        self.user.room = self.tui.get_input(&self.state)?;

        Ok(())
    }

    pub async fn subscribe(&mut self) -> Result<()> {
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

    pub fn close(&mut self, err: Option<Error>) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            self.tui.terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            event::PopKeyboardEnhancementFlags
        )?;
        self.tui.terminal.show_cursor()?;
        if let Some(err) = err {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Ok(())
    }
}

pub fn new() -> Result<App> {
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
        tui: tui::TUI { terminal },
        state: State::Username,
        user: User {
            username: String::new(),
            room: String::new(),
            token: String::new(),
        },
    })
}
