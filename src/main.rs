use sis_login::Sis;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let headers_builder = sis_login::headers_builder::DefaultHeadersBuilder::new(
        "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0".to_string(),
        "https://sis.eelu.edu.eg/static/PortalStudent.html".to_string(),
    );
    let mut sis = Sis::new(
        "https://sis.eelu.edu.eg/studentLogin",
        "https://sis.eelu.edu.eg/getJCI",
        &headers_builder,
    );
    cli::cli_session::login(&mut sis).await;
    Ok(())
}
