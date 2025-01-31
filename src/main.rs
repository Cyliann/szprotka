mod app;
mod error;
mod prelude;
mod tui;
mod web;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let result = app::App::default().run().await;

    match result {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
