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
        self.get_input(false)?;

        // retry if form submition is invalid
        while let Err(err) = self.subscribe().await {
            match err {
                Error::InvalidForm => self.get_input(true),
                _ => Err(err),
            }?
        }
        self.receive_messages().await?;

        Ok(())
    }

    fn get_input(&mut self, invalid_form: bool) -> Result<()> {
        (self.user.username, self.user.room) =
            tui::forms::LoginForm::default().run(&mut self.terminal, invalid_form)?;

        Ok(())
    }

    async fn subscribe(&mut self) -> Result<()> {
        let client = reqwest::Client::new();

        match web::req::register(&client, &self.user.username, &self.user.room).await {
            Ok((token, room)) => {
                self.user.room = room.clone();
                self.user.token = token.clone();

                let message_lock = self.messages.clone();
                tokio::task::spawn(async move { web::sse::handle_sse(token, message_lock).await });
                Ok(())
            }
            Err(err) => match err {
                Error::Request(err) => match err.status() {
                    Some(reqwest::StatusCode::BAD_REQUEST) => Err(Error::InvalidForm),
                    _ => Err(err.into()),
                },
                _ => Err(err),
            },
        }
    }

    async fn receive_messages(&mut self) -> Result<()> {
        tui::sse::MessageReceiver::default()
            .run(
                &mut self.terminal,
                self.user.room.clone(),
                self.messages.clone(),
                self.user.token.clone(),
            )
            .await?;
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
