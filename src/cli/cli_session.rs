use super::super::sis::{login::moodle_login, types::user_type::UserType};
use super::arg_parser::Arguments;

pub async fn login() {
    let mut args: Arguments = Arguments::parse_args();
    loop {
        let mut i = 1;
        let moodle_session_url = loop {
            if args.user_type.is_none() {
                args.user_type = Some(UserType::from(i));
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
                    if i == 3 {
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
                return;
            }
            None => {
                println!("[-] Login Faild :(");
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
