use crate::prelude::*;
use crate::tui;
use crate::web;
use std::io::Stdout;
use std::sync::Arc;
use std::sync::Mutex;

use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;

#[derive(Default)]
pub struct App {
    tui: tui::TUI,
    state: State,
    pub user: User,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Default)]
pub struct User {
    pub username: String,
    pub room: String,
    token: String,
}

#[derive(Default)]
pub enum State {
    #[default]
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
}

impl Drop for App {
    fn drop(&mut self) {
        cleanup();
    }
}

pub fn cleanup() {
    ratatui::restore();
}
