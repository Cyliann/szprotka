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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let (username, room) = handle_input();
    let (token, room) = register(&client, username, room)?;

    println!("Room: {}", room);
    println!("Token: {}", token);
    Ok(())
}

fn register(
    client: &reqwest::blocking::Client,
    username: String,
    _room: String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    let res_body: RegisterRespone;

    params.insert("username", username);
    // TODO
    params.insert("room", String::from(""));

    let resp = client
        .post("http://localhost:8080/register")
        .json(&params)
        .send()?
        .error_for_status();

    if let Err(err) = resp {
        return Err(err.status().unwrap().to_string())?;
    }

    let ok_resp = resp?;
    if let Some(auth) = ok_resp.headers().get(AUTHORIZATION) {
        let token = String::from_utf8_lossy(auth.as_bytes()).to_string();
        res_body = ok_resp.json()?;
        return Ok((token, res_body.room));
    }
    Err("Client unauthorized")?
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
    return (username, room);
}
