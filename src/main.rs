use std::{
    thread::sleep,
    time::Duration,
};

mod sse;
mod web;
mod input;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let (username, room) = input::handle_input();
    let (token, room) = web::register(&client, username, room).await?;
    println!("Room: {}", room);

    let token_clone = token.clone();
    tokio::task::spawn(async move {
        let _ = sse::handle_sse(token_clone).await;
    });

    loop {
        web::roll(&client, &token).await?;
        if false {
            break;
        }
        sleep(Duration::from_secs(3));
    }

    println!("Room: {}", room);
    Ok(())
}
