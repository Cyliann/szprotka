mod web;
mod input;
mod app;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::new();
    app.run().await?;
    Ok(())
}
