use crate::cli::arg_parser::Arguments;
use sis_login::sis::types::user_type::UserType;
use std::io::{stderr, stdin, stdout, Write};

pub async fn login(sis: &mut sis_login::Sis<'_>, args: &mut Arguments) {
    let try_guess_user_type = args.user_type.is_none();

    if args.verbose && try_guess_user_type {
        println!("[+] The User Type is not specified, Trying to guess it ...");
        println!("[!] This may take a while ...");
    }

    loop {
        // add on order &slice[student , staff , system-user ] because most users of this tool will
        // be students so we start with them
        for user_type_num in [2, 3, 1] {
            if try_guess_user_type {
                let user_type = UserType::from(user_type_num);
                args.user_type = Some(user_type.clone());
                if args.verbose {
                    println!("[+] Trying login as {}", user_type.to_string());
                }
            }
            // Try Login
            match sis
                .login(
                    &args.username.clone().unwrap(),
                    &args.password.clone().unwrap(),
                    args.user_type.clone().unwrap(),
                )
                .await
            {
                Ok(_) => {
                    if args.verbose {
                        println!("[+] Login Success :)");
                    }
                    break;
                }
                _ => {
                    if args.verbose {
                        println!("[-] Login Faild :(");
                    }
                    if !try_guess_user_type {
                        break;
                    }
                }
            }
        }

        match sis.get_moodle_session().await {
            Ok(url) => {
                if args.verbose {
                    println!("[+] Moodle URL : {}", url);
                } else if !args.open_browser {
                    println!("{}", url);
                }

                // Open the Moodle URL in the default browser if the user wants to
                if args.open_browser {
                    if let Err(err) = open::that(url) {
                        println!("[-] Failed To Open Browser: {err}");
                    };
                }
                if args.verbose {
                    prompt_enter("\n\nPlease send blessings upon Prophet Muhammad Then Press Enter To Exit\n\n");
                }

                return;
            }
            _ => {
                if prompt_y_n("[yes/no] => Do You Want to Attemp Login Again ?") {
                    if prompt_y_n("[yes/no] => Do You Want to Login Useing Same User And Pass ?") {
                        continue;
                    } else {
                        args.username = prompt("Username", true).into();
                        args.password = prompt("Password", true).into();
                        continue;
                    }
                } else {
                    return;
                }
            }
        }
    }
}

pub fn prompt_y_n(msg: &str) -> bool {
    match prompt(msg, false)
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

#[inline]
pub fn prompt(param_name: &str, empty_input_err: bool) -> String {
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

#[inline(always)]
pub fn banner() {
    println!(
        r#"
0xCrypt00o Moodle Login - Tool for Getting Moodle Session Quickly and Login Through CLI

    Created By : Eslam Muhammad [0xCrypt00o]
    Github Repo : https://github.com/Crypt00o/eelu-login
    Mail : 0xCrypt00o@protonmail.com
    Support Me On :
        Bitcoin Address : bc1qdp3f6u3puwkyu6ztxz7hzcurd7jnjyw6hzjcas"#
    );
}
