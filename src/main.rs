use crate::prelude::*;

mod app;
mod error;
mod prelude;
mod tui;
mod web;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let mut app = app::App::default();
    app.get_input()?;
    app.subscribe().await?;

    Ok(())
}
