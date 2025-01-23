use crate::prelude::*;
use crate::web;
use crate::{error::Error, tui};
use std::sync::Arc;
use std::sync::Mutex;

use crossterm::{
    event::{self},
    execute,
    terminal::disable_raw_mode,
};

pub struct App {
    tui: tui::TUI,
    state: State,
    pub user: User,
    messages: Arc<Mutex<Vec<String>>>,
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
        self.user.room = room.clone();
        self.user.token = token.clone();

        let message_lock = self.messages.clone();
        tokio::task::spawn(async move { web::sse::handle_sse(token, message_lock).await });

        self.tui.display_sse(room, self.messages.clone())?;

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
    Ok(App {
        tui: tui::new()?,
        state: State::Username,
        user: User {
            username: String::new(),
            room: String::new(),
            token: String::new(),
        },
        messages: Arc::new(Mutex::new(vec![])),
    })
}
