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
    result: Vec<u8>,
}

pub async fn handle_sse(token: String) -> Result<(), eventsource_client::Error> {
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
                handle_events(ev);
            }
            es::SSE::Comment(comment) => {
                println!("got a comment: \n{}", comment)
            }
        })
        .map_err(|err| eprintln!("error streaming events: {:?}", err))
}

fn handle_events(ev: es::Event) {
    if ev.event_type != "roll" {
        println!("Event type: {}\nEvent data: {}", ev.event_type, ev.data);
        return
    }

    let body: Roll = serde_json::from_str(&ev.data).unwrap();

    let mut results = String::new();
    for r in body.result.iter() {
        results.push_str(&r.to_string());
        results.push_str(", ");
    }
    println!("{} rolled {}", body.username, results);
}
