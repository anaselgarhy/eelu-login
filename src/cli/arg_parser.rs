use std::env::{Args,args};
use::std::io::{stdin,stdout,stderr,Write};

#[derive(Debug)]
pub struct Arguments{
  pub   username:Option<String>,
  pub   password:Option<String>,
  pub   usertype:Option<String>
}


impl Arguments {  

    pub fn new()->Self{
        return Self { 
            username:None ,
            password: None,
            usertype: None
        };
    }

    fn prompt(param_name:&str)->String{
        loop{
            let mut prompt_buf:String=String::new();
            print!("Enter {} : ",param_name);
            stdout().flush().unwrap();

            match stdin().read_line(&mut prompt_buf) {
                Ok(_)=>{
                    prompt_buf.pop();
                    if !prompt_buf.is_empty(){
                        return prompt_buf;
                    }
                    else{
                        eprintln!("[-] Empty {} !",param_name);
                        stderr().flush().unwrap();
                        continue;
                    }
                },
                
                Err(error)=>{
                    eprintln!("[-] Error While Reading {} : {} \n [!] Repeating",param_name,error);
                    stderr().flush().unwrap();
                    continue;
                }
            };
    
        };
    }

    pub fn read_needed_arguments(mut self)->Self{
        if self.username.is_none(){
            self.username=Some(Self::prompt("Username"));    
        }
        if self.password.is_none(){
            self.password=Some(Self::prompt("Password"));
        }
        if self.usertype.is_none(){
            match Self::prompt("Usertype").to_lowercase().as_str() {
                "0"|"admin"=>self.usertype=Some(String::from("admin")),
                "1"|"system-user"|"systemuser"|"system user"|"sys-user"|"sysuser"|"sys user"=>self.usertype=Some(String::from("system-user")),
                "2"|"student"=>self.usertype=Some(String::from("student")),
                _=>self.usertype=Some(2.to_string())
            }
        }
        return self;
    }
    
    pub fn prompt_y_n(msg:&str)->bool{
       match Self::prompt(msg).to_lowercase().trim_end_matches(char::is_whitespace) {
                        "y"|"yes"=>return true,
                        "n"|"no"=>return false,
                        _=>return  false
        };
    }

    pub fn parse_args()->Self{
        let mut cli_args:Args=args();
        let mut index:Option<String>=cli_args.next();
        let mut parsed_arguments:Self=Self::new();
    
        loop{
            index=cli_args.next();
            if index.is_some(){
                match index.as_ref().unwrap().as_str() {
                    "--user" | "--username" | "-user" | "-u" => parsed_arguments.username=cli_args.next(),
                    "--password"|"-password"|"--pass"|"-pass"|"-p" => parsed_arguments.password=cli_args.next(),
                    "--usertype"|"-usertype"|"--type"|"-type"|"-t"=> parsed_arguments.usertype=cli_args.next(),
                    _=> println!("Invalid Argument : {}",index.unwrap())
                }
            } 
            else{
              return parsed_arguments.read_needed_arguments();
            }
        }
    }
}

