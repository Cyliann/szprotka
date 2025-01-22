use std::error::Error;

mod app;
mod error;
mod prelude;
mod web;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = app::new()?;
    app.get_input()?;
    app.close(None)?;

    println!("Username: {}", app.user.username);
    app.subscribe().await?;

    Ok(())
}
