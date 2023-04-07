use sis_login::sis::types::user_type::UserType;
use crate::cli::arg_parser::Arguments;

pub async fn login(sis: &mut sis_login::Sis<'_>) {
    let mut args: Arguments = Arguments::parse_args();
    let try_guess_user_type = args.user_type.is_none();
    loop {
        // add on order &slice[student , staff , system-user ] because most users of this tool will
        // be students so we start with them
        for user_type_num in [2, 3, 1] {
            if try_guess_user_type {
                args.user_type = Some(UserType::from(user_type_num));
            }
            // Try Login
            match sis.login(&args.username.clone().unwrap(),
                            &args.password.clone().unwrap(),
                            args.user_type.clone().unwrap()).await {
                Ok(_) => {
                    break;
                }
                _ => {
                    println!("[-] Login Faild :(");
                    if !try_guess_user_type {
                        break;
                    }
                }
            }
        };

        match sis.get_moodle_session().await {
            Ok(url) => {
                println!("[+] Moodle URL : {}", url);
                Arguments::prompt_enter(
                    "\n\nPlease send blessings upon Prophet Muhammad Then Press Enter To Exit\n\n",
                );
                return;
            }
            _ => {
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
