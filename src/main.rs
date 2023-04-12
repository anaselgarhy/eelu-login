//! # eelu-login
//!
//! eelu-login is a command-line tool that allows users to log in to the EELU Moodle platform quickly and easily through the command-line interface.
//!
//! ## Installation For Users
//! You can install the latest stable version of eelu-login via Cargo:
//! `cargo install eelu-login`
//!
//! Or you can get the latest git version from the repository:
//! `cargo install --git https://github.com/anaselgarhy/eelu-login.git`
//!
//! Or check [Releases page](https://github.com/Crypt00o/eelu-login/releases) for pre-built binaries (not updated frequently).
//!
//! ## Installation and Building For Developers
//! To install eelu-login, you'll need to have Rust and Cargo installed on your machine. Once you have those installed, you can run the following command:
//!
//! build:
//! ```sh
//! cargo build --release
//! ```
//!
//! run :
//! ```sh
//! cargo run
//! ```
//!
//! ## Usage
//! `eelu-login --help` will show you the usage of the tool:
//! ```text
//! [+] Usage : eelu-login [--user <username>] [--pass <password>] [--type <staff| sys-user | student>]
//! Args:
//! [-user | --user | --username | -username |  -u]   <username>  :  username to login with
//! [-pass | --pass | --password | -p]   <password>  :  password to login with
//! [-type | --type | --usertype | -usertype | -t]  : <usertype>
//!
//! Flags:
//! [-o | --open | -open] : open browser after login
//! [-v | --verbose | -verbose] : verbose mode
//! [-V | --version | -version] : print version
//! [-h | --help | -help] : print this help message
//!
//! usertype can be :
//!     [ staff | 3 ] for staff privilege
//!     [ sys-user | 1] for system user privilege
//!     [ student | 2] for student privilege"#
//! ```
//!
//! Replace `<username>` and `<password>` with your EELU Moodle login credentials, and `< staff | sys-user | student>` with your user type.
//!
//! If you don't want to enter your credentials every time you run the tool, you can set the `SIS_EELU_USERNAME` and `SIS_EELU_PASSWORD` environment variables to your username and password respectively.
//!
//! You can don't need to specify the user, and the tool will be try to login as a student and if it fails it will try to login as a staff and if it fails it will try to login as a system user.
//!
//! ## Support
//! If you have any questions or need help using eelu-login, you can contact the creator, Eslam Muhammad [0xCrypt00o], at 0xCrypt00o@protonmail.com. You can also support the creator by sending Bitcoin to the following address: bc1qdp3f6u3puwkyu6ztxz7hzcurd7jnjyw6hzjcas.
//!
//!
//! ## Contributors 🦀❤
//! - [Anas Elgarhy](https://github.com/anaselgarhy)
//!
//! ## Contributing
//! If you want to contribute to this project, please read the [contributing guidelines](./CONTRIBUTING.md) first.
//!
//! ## Useful Links
//! - [sis-login crate](https://crates.io/crates/sis-login)
//! - [EELU SIS](https://sis.eelu.edu.eg/)
//! - [EELU Moodle](https://moodle.eelu.edu.eg/)
//!
//! ## License
//! This package is licensed under the [MIT License](./LICENSE) Or [Apache License 2.0](./LICENSE-apache) (c) 2023 Eslam Muhammad [0xCrypt00o] and contributors.
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
