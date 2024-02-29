use std::{
    thread::sleep,
    time::Duration,
};

use crate::input;
use crate::web;

pub struct App {
    should_quit: bool
}

impl App {
    pub fn new() -> App {
        App{should_quit: false}
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let (username, room) = input::handle_input();
        let (token, room) = web::req::register(&client, username, room).await?;
        println!("Room: {}", room);

        let token_clone = token.clone();
        tokio::task::spawn(async move {
            let _ = web::sse::handle_sse(token_clone).await;
        });

        loop {
            web::req::roll(&client, &token).await?;
            if self.should_quit {
                break;
            }
            sleep(Duration::from_secs(3));
        }
        Ok(())
    }
}
