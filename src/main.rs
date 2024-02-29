use std::{
    thread::sleep,
    time::Duration,
};

mod web;
mod input;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let (username, room) = input::handle_input();
    let (token, room) = web::req::register(&client, username, room).await?;
    println!("Room: {}", room);

    let token = token.clone();
    tokio::task::spawn(async move {
        let _ = web::sse::handle_sse(token).await;
    });

    loop {
        web::req::roll(&client, &token).await?;
        if false {
            break;
        }
        sleep(Duration::from_secs(3));
    }

    println!("Room: {}", room);
    Ok(())
}
