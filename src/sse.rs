use eventsource_client as es;
use futures::{Stream, TryStreamExt};

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
                println!("got an event: {}\n{}", ev.event_type, ev.data)
            }
            es::SSE::Comment(comment) => {
                println!("got a comment: \n{}", comment)
            }
        })
        .map_err(|err| eprintln!("error streaming events: {:?}", err))
}
