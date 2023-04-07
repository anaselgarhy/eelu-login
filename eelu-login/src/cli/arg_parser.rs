use std::env::{args, Args};
use std::io::{stderr, stdin, stdout, Write};
use std::process::exit;
use sis_login::sis::types::user_type::UserType;

pub struct Arguments {
    pub username: Option<String>,
    pub password: Option<String>,
    pub user_type: Option<UserType>,
}

impl Arguments {
    pub fn new() -> Self {
        Self {
            username: None,
            password: None,
            user_type: None,
        }
    }

    fn prompt(param_name: &str, empty_input_err: bool) -> String {
        loop {
            let mut prompt_buf: String = String::new();
            print!("Enter {} : ", param_name);
            stdout().flush().unwrap();

            match stdin().read_line(&mut prompt_buf) {
                Ok(_) => {
                    prompt_buf.pop();
                    if prompt_buf.is_empty() && empty_input_err {
                        eprintln!("[-] Empty {} !", param_name);
                        stderr().flush().unwrap();
                        continue;
                    } else {
                        return prompt_buf;
                    }
                }

                Err(error) => {
                    eprintln!(
                        "[-] Error While Reading {} : {} \n [!] Repeating",
                        param_name, error
                    );
                    stderr().flush().unwrap();
                    continue;
                }
            };
        }
    }

    pub fn read_needed_arguments(mut self) -> Self {
        if self.username.is_none() {
            self.username = Some(Self::prompt("Username", true));
        }
        if self.password.is_none() {
            self.password = Some(Self::prompt("Password", true));
        }
        self
    }

    pub fn prompt_y_n(msg: &str) -> bool {
        match Self::prompt(msg, false)
            .to_lowercase()
            .trim_end_matches(char::is_whitespace)
        {
            "y" | "yes" => true,
            "n" | "no" => false,
            _ => true,
        }
    }

    pub fn prompt_enter(msg: &str) {
        print!("{} ", msg);
        stdout().flush().unwrap();
        stdin().read_line(&mut String::new());
    }

    fn banner() {
        println!(
            r"

0xCrypt00o Moodle Login - Tool for Getting Moodle Session Quickly and Login Through CLI
    
    Created By : Eslam Muhammad [0xCrypt00o]
    Github Repo : https://github.com/Crypt00o/eelu-login
    Mail : 0xCrypt00o@protonmail.com
    Support Me On :
        Bitcoin Address : bc1qdp3f6u3puwkyu6ztxz7hzcurd7jnjyw6hzjcas

                 "
        );
    }

    fn usage() {
        println!(
            r"
[+] Usage : eelu-login --user <username> --pass <password> --type <staff| sys-user | student >

Args: 

[-user | --user | --username | -username |  -u]   <username>  :  username to login with 
[-pass | --pass | --password | -p]   <password>  :  password to login with
[-type | --type | --usertype | -usertype | -t]  : <usertype> 

usertype can be :
    [ staff | 3 ] for staff privilege
    [ sys-user | 1] for system user privilege
    [ student | 2] for student privilege
                 
    "
        );
    }
    pub fn parse_args() -> Self {
        Self::banner();
        let mut cli_args: Args = args();
        let mut index: Option<String> = cli_args.next();
        let mut parsed_arguments: Self = Self::new();

        loop {
            index = cli_args.next();
            if index.is_some() {
                match index.as_ref().unwrap().as_str() {
                    "--user" | "-username" | "--username" | "-user" | "-u" => {
                        parsed_arguments.username = cli_args.next()
                    }
                    "--password" | "-password" | "--pass" | "-pass" | "-p" => {
                        parsed_arguments.password = cli_args.next()
                    }
                    "--usertype" | "-usertype" | "--type" | "-type" | "-t" => {
                        if let Some(user_type) = cli_args.next() {
                            parsed_arguments.user_type = Some(UserType::from_string(&user_type));
                        }
                    }
                    "-h" | "-help" | "--help" => {
                        Self::usage();
                        exit(0)
                    }
                    _ => println!("Invalid Argument : {}", index.unwrap()),
                }
            } else {
                return parsed_arguments.read_needed_arguments();
            }
        }
    }
}
