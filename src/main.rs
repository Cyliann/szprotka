use std::error::Error;

mod app;
mod error;
mod prelude;
mod tui;
mod web;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = app::new()?;
    if let Err(err) = app.get_input() {
        app.close(Some(err))?;
    }

    if let Err(err) = app.subscribe().await {
        app.close(Some(err))?;
    }
    app.close(None)?;

    Ok(())
}
