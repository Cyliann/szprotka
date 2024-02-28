use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

mod sse;
mod web;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let (username, room) = handle_input();
    let (token, room) = web::register(&client, username, room).await?;

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

fn handle_input() -> (String, String) {
    let room = String::new();
    let mut username = String::new();

    print!("Username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();

    // print!("Room (can be empty): ");
    // io::stdout().flush()?;
    // io::stdin().read_line(&mut room)?;
    //
    (username, room)
}
