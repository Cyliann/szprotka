use reqwest::blocking::Client;
use serde_json;
use std::error::Error;

mod app;
mod error;

const URL: &str = "http:localhost:8080";

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = app::new()?;
    let (username, room) = app.run()?;
    app.close(None)?;

    println!("Username: {}", username);
    println!("Room: {}", room);

    // Perform HTTP Request
    let client = Client::new();
    let response = client
        .post(format!("{}/register", URL))
        .json(&serde_json::json!({ "username": username, "room": room }))
        .send()?;

    println!("Response: {:?}", response.text()?);

    Ok(())
}
