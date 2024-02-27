use futures::{Stream,TryStreamExt};
use eventsource_client as es;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Deserialize)]
struct RegisterRespone {
    #[serde(rename = "Room")]
    room: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let (username, room) = handle_input();
    let (token, room) = register(&client, username, room).await?;
    println!("Token: {}", &token);
    handle_sse(token).await?;

    println!("Room: {}", room);
    Ok(())
}

async fn register(
    client: &reqwest::Client,
    username: String,
    _room: String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    let res_body: RegisterRespone;

    params.insert("username", username);
    // TODO
    params.insert("room", String::from(""));

    let resp_result = client
        .post("http://localhost:8080/register")
        .json(&params)
        .send()
        .await?
        .error_for_status();

    if let Err(err) = resp_result {
        return Err(err.status().unwrap().to_string())?;
    }

    let resp = resp_result?;
    println!("{resp:#?}");
    
    if let Some(auth) = resp.headers().get(AUTHORIZATION) {
        let token = String::from_utf8_lossy(auth.as_bytes()).to_string();
        res_body = resp.json().await?;
        return Ok((token, res_body.room));
    }
    Err("Error obtaining the token")?
}

fn handle_input() -> (String, String) {
    let room = String::new();
    let mut username = String::new();

    print!("Username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    // print!("Room (can be empty): ");
    // io::stdout().flush()?;
    // io::stdin().read_line(&mut room)?;
    //
    (username, room)
}

async fn handle_sse(token: String) -> Result<(), eventsource_client::Error> {
let token_str = format!("Bearer {token}");
let client = eventsource_client::ClientBuilder::for_url("http://localhost:8080/play")?
    .header("Authorization", token_str.as_str())?
    .build();

let mut stream = tail_events(client);

    while let Ok(Some(_)) = stream.try_next().await {}

Ok(())
}

fn tail_events(client: impl es::Client) -> impl Stream<Item = Result<(), ()>> {
    client
        .stream()
        .map_ok(|event| match event {
            es::SSE::Event(ev) => {
                println!("got an event: {}\n{}", ev.event_type, ev.data)
            }
            es::SSE::Comment(comment) => {
                println!("got a comment: \n{}", comment)
            }
        })
        .map_err(|err| eprintln!("error streaming events: {:?}", err))
}
