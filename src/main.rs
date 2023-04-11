use sis_login::Sis;

mod cli;

use cli::cli_session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Parse Arguments
    let mut args = cli::arg_parser::Arguments::parse_args_and_env();

    // Print the banner if the verbose flag is set
    if args.verbose {
        cli_session::banner();
    }

    // Check the user entered the username and password, if not, prompt the user to enter them
    if args.username.is_none() {
        args.username = cli_session::prompt("Username", true).into();
    }
    if args.password.is_none() {
        args.password = cli_session::prompt("Password", true).into();
    }

    if args.verbose {
        println!("[+] Preparing to login ...");
        println!("    [+] Username: {}", args.username.clone().unwrap());
        println!("    [+] Password: {}", args.password.clone().unwrap());
        println!("[+] Initializing the SIS session ...");
    }

    let headers_builder = sis_login::headers_builder::DefaultHeadersBuilder::new(
        "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0".to_string(),
        "https://sis.eelu.edu.eg/static/PortalStudent.html".to_string(),
    );
    let mut sis = Sis::new(
        "https://sis.eelu.edu.eg/studentLogin",
        "https://sis.eelu.edu.eg/getJCI",
        &headers_builder,
    );

    if args.verbose {
        println!("[+] SIS session initialized successfully");
        println!("[+] Login ...");
    }

    cli_session::login(&mut sis, &mut args).await;
    Ok(())
}
