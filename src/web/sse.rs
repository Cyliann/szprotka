use std::sync::{Arc, Mutex};

use crate::prelude::*;

use eventsource_client as es;
use futures::{Stream, TryStreamExt};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct Roll {
    #[serde(rename = "Username")]
    username: String,
    #[serde(rename = "Room")]
    _room: String,
    #[serde(rename = "Result")]
    result: u8,
}

const URL: &str = "http://localhost:8080";

pub async fn handle_sse(token: String, message_lock: Arc<Mutex<Vec<String>>>) -> Result<()> {
    let token_str = format!("Bearer {token}");
    let client = eventsource_client::ClientBuilder::for_url(&format!("{}/play", URL))?
        .header("Authorization", token_str.as_str())?
        .build();

    let mut stream = tail_events(client, message_lock);

    loop {
        let res = stream.try_next().await;
        match res {
            Ok(Some(_)) => (),
            Ok(None) => break,
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

fn tail_events(
    client: impl es::Client,
    message_lock: Arc<Mutex<Vec<String>>>,
) -> impl Stream<Item = Result<()>> {
    client
        .stream()
        .map_ok(move |event| {
            let message = match event {
                es::SSE::Event(ev) => handle_events(ev),
                es::SSE::Comment(comment) => {
                    format!("Comment: \n{}", comment)
                }
                es::SSE::Connected(conn) => {
                    format!("Connected! {}", conn.response().status())
                }
            };
            message_lock.lock().unwrap().push(message);
        })
        .map_err(|err| return err.into())
}

fn handle_events(ev: es::Event) -> String {
    if ev.event_type != "roll" {
        return format!("Event type: {}\nEvent data: {}", ev.event_type, ev.data);
    }

    let body: Roll = serde_json::from_str(&ev.data).unwrap();

    format!("{} rolled {}", body.username, body.result)
}
