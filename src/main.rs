mod cli;
mod sis;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::cli_session::login().await;
    Ok(())
}
