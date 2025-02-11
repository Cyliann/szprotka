use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;

use std::{
    io,
    sync::{Arc, Mutex},
};

use crate::prelude::*;
use crate::tui;
use crate::web;

pub struct App {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub user: User,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Default)]
pub struct User {
    pub username: String,
    pub room: String,
    token: String,
}

impl App {
    pub async fn run(&mut self) -> Result<()> {
        self.get_input()?;
        self.subscribe().await?;
        self.receive_messages()?;

        Ok(())
    }

    fn get_input(&mut self) -> Result<()> {
        (self.user.username, self.user.room) =
            tui::forms::LoginForm::default().run(&mut self.terminal)?;

        Ok(())
    }

    async fn subscribe(&mut self) -> Result<()> {
        let client = reqwest::Client::new();

        let (token, room) =
            web::req::register(&client, &self.user.username, &self.user.room).await?;
        self.user.room = room.clone();
        self.user.token = token.clone();

        let message_lock = self.messages.clone();
        tokio::task::spawn(async move { web::sse::handle_sse(token, message_lock).await });

        Ok(())
    }

    fn receive_messages(&mut self) -> Result<()> {
        tui::sse::MessageReceiver::default().run(
            &mut self.terminal,
            self.user.room.clone(),
            self.messages.clone(),
        )?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        let terminal = ratatui::init();
        Self {
            terminal,
            user: User::default(),
            messages: Arc::new(Mutex::new(vec![])),
        }
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
