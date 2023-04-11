use sis_login::sis::types::user_type::UserType;
use std::env::{args, Args};
use std::io::{stderr, stdin, stdout, Write};
use std::process::exit;

pub struct Arguments {
    pub username: Option<String>,
    pub password: Option<String>,
    pub user_type: Option<UserType>,
    pub open_browser: bool,
    pub verbose: bool,
}

impl Arguments {
    pub fn new() -> Self {
        Self {
            username: None,
            password: None,
            user_type: None,
            open_browser: false,
            verbose: false,
        }
    }



    #[inline(always)]
    fn usage() {
        println!(r#"[+] Usage : eelu-login [--user <username>] [--pass <password>] [--type <staff| sys-user | student>]
Args:
[-user | --user | --username | -username |  -u]   <username>  :  username to login with 
[-pass | --pass | --password | -p]   <password>  :  password to login with
[-type | --type | --usertype | -usertype | -t]  : <usertype>


Flags:
[-o | --open | -open] : open browser after login
[-v | --verbose | -verbose] : verbose mode
[-V | --version | -version] : print version
[-h | --help | -help] : print this help message

usertype can be :
    [ staff | 3 ] for staff privilege
    [ sys-user | 1] for system user privilege
    [ student | 2] for student privilege"#);
    }

    #[inline(always)]
    fn parse_args(&mut self) {
        let mut cli_args: Args = args();
        let mut index: Option<String> = cli_args.next();

        loop {
            index = cli_args.next();
            if index.is_some() {
                match index.as_ref().unwrap().as_str() {
                    "--user" | "-username" | "--username" | "-user" | "-u" => {
                        self.username = cli_args.next()
                    }
                    "--password" | "-password" | "--pass" | "-pass" | "-p" => {
                        self.password = cli_args.next()
                    }
                    "--usertype" | "-usertype" | "--type" | "-type" | "-t" => {
                        if let Some(user_type) = cli_args.next() {
                            self.user_type = Some(UserType::from_string(&user_type));
                        }
                    }
                    "-o" | "-open" | "--open" => self.open_browser = true,
                    "-v" | "-verbose" | "--verbose" => self.verbose = true,
                    "-V" | "-version" | "--version" => {
                        println!("eelu-login v{}", env!("CARGO_PKG_VERSION"));
                        exit(0)
                    }
                    "-h" | "-help" | "--help" => {
                        Self::usage();
                        exit(0)
                    }
                    _ => println!("Invalid Argument : {}", index.unwrap()),
                }
            } else {
                break;
            }
        }
    }

    #[inline]
    pub fn parse_args_and_env() -> Self {
        let mut parsed_arguments: Self = Self::new();

            // Try to get username from env var
            if let Ok(username) = std::env::var("EELU_SIS_USERNAME") {
                parsed_arguments.username = Some(username);
            }

            // Try to get password from env var
            if let Ok(password) = std::env::var("EELU_SIS_PASSWORD") {
                parsed_arguments.password = Some(password);
            }

        parsed_arguments.parse_args();
        parsed_arguments
    }
}
