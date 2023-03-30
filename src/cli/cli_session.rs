use super::arg_parser::Arguments;
use super::super::sis::{login::moodle_login,types::user_type::UserType};


pub async fn login(){
    let mut args:Arguments=Arguments::parse_args();
    loop{        
            let moodle_session_url:Option<String>=moodle_login(&args.username.clone().unwrap(),&args.password.clone().unwrap(), UserType::from_string(&args.usertype.clone().unwrap())).await;
            match moodle_session_url {
                Some(url)=>{
                   println!("[+] Moodle URL : {}",url); 
                    return ();
                },
                None=>{
                    println!("[-] Login Faild :(");
                    if Arguments::prompt_y_n("[yes/no] => Do You Want to Attemp Login Again ?"){
                        if Arguments::prompt_y_n("[yes/no] => Do You Want to Login Useing Same User And Pass ?"){
                            continue;
                        }else{
                            args=Arguments::new().read_needed_arguments();
                            continue;
                        }
                    }
                    else{
                        return ();
                    }

                }
            }
        } 
}
