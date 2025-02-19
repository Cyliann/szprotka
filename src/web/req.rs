use crate::prelude::*;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct RegisterRespone {
    #[serde(rename = "Room")]
    room: String,
}

const URL: &str = "http://localhost:8080";

pub async fn register(
    client: &reqwest::Client,
    username: &String,
    room: &String,
) -> Result<(String, String)> {
    let mut params = HashMap::new();
    let res_body: RegisterRespone;

    params.insert("username", username);
    params.insert("room", room);

    let resp_result = client
        .post(format!("{}/register", URL))
        .json(&params)
        .send()
        .await?
        .error_for_status();

    if let Err(err) = resp_result {
        return Err(err)?;
    }

    let resp = resp_result?;

    if let Some(auth) = resp.headers().get(AUTHORIZATION) {
        let token = String::from_utf8_lossy(auth.as_bytes()).to_string();
        res_body = resp.json().await?;
        return Ok((token, res_body.room));
    }
    Err(Error::Token)
}

pub async fn roll(client: &reqwest::Client, token: &String, dice: u8) -> Result<reqwest::Response> {
    let mut params = HashMap::new();
    let mut headers = reqwest::header::HeaderMap::new();

    params.insert("dice", dice);
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());

    let req = client
        .post(format!("{}/roll", URL))
        .headers(headers)
        .json(&params)
        .send()
        .await?
        .error_for_status();

    match req {
        Ok(resp) => Ok(resp),
        Err(err) => Err(err.into()),
    }
}
