mod utils;
mod sis;
mod cli;


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
   cli::cli_session::login().await;    
   return Ok(());
}
