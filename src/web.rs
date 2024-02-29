use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct RegisterRespone {
    #[serde(rename = "Room")]
    room: String,
}

pub async fn register(
    client: &reqwest::Client,
    username: String,
    room: String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    let res_body: RegisterRespone;

    params.insert("username", username);
    params.insert("room", room);

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

    if let Some(auth) = resp.headers().get(AUTHORIZATION) {
        let token = String::from_utf8_lossy(auth.as_bytes()).to_string();
        res_body = resp.json().await?;
        return Ok((token, res_body.room));
    }
    Err("Error obtaining the token")?
}

pub async fn roll(client: &reqwest::Client, token: &String) -> Result<(), reqwest::Error> {
    let mut params = HashMap::new();
    let mut headers = reqwest::header::HeaderMap::new();

    params.insert("dice", [100, 20, 6]);
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());

    let req = client
        .post("http://localhost:8080/roll")
        .headers(headers)
        .json(&params)
        .send()
        .await?
        .error_for_status();

    if let Err(err) = req {
        return Err(err);
    }

    Ok(())
}
