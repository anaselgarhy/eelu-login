use super::super::sis::{login::moodle_login, types::user_type::UserType};
use super::arg_parser::Arguments;

pub async fn login() {
    let mut args: Arguments = Arguments::parse_args();
    let try_guess_user_type = args.user_type.is_none();
    loop {
        // add on order &slice[student , staff , system-user ] because most users of this tool will
        // be students so we start with them
        let usertypes: &[u8] = &[2,3,1];
        let mut i = 0;
        let moodle_session_url = loop {
            if args.user_type.is_none() && try_guess_user_type {
                args.user_type = Some(UserType::from(usertypes[i]));
            }
            // Try Login
            let moodle_session_url: Option<String> = moodle_login(
                &args.username.clone().unwrap(),
                &args.password.clone().unwrap(),
                args.user_type.clone().unwrap(),
            )
            .await;
            match moodle_session_url {
                Some(_) => {
                    break moodle_session_url;
                }
                _ => {
                    println!("[-] Login Faild :(");
                    if i == 2 || !try_guess_user_type {
                        break None;
                    }
                    
                    i += 1;
                    args.user_type = None;
                }
            }
        };

        match moodle_session_url {
            Some(url) => {
                println!("[+] Moodle URL : {}", url);
                Arguments::prompt_enter("\n\nPlease send blessings upon Prophet Muhammad Then Press Enter To Exit\n\n");
                return;
            }
            None => {
                if Arguments::prompt_y_n("[yes/no] => Do You Want to Attemp Login Again ?") {
                    if Arguments::prompt_y_n(
                        "[yes/no] => Do You Want to Login Useing Same User And Pass ?",
                    ) {
                        continue;
                    } else {
                        args = Arguments::new().read_needed_arguments();
                        continue;
                    }
                } else {
                    return;
                }
            }
        }
    }
}
