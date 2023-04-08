# sis-login

A library to login to the sis system and get the moodle session

## Example
```rust
use sis_login::Sis;
use sis_login::sis::types::user_type::UserType;

#[tokio::main]
async fn main() {
   let username = std::env::var("SIS_USERNAME").unwrap();
   let password = std::env::var("SIS_PASSWORD").unwrap();

   // Crate Sis instance
   let headers_builder = sis_login::headers_builder::DefaultHeadersBuilder::new(
      "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0".to_string(),
     "https://sis.eelu.edu.eg/static/PortalStudent.html".to_string()
  );

   let login_url: &str = "https://sis.eelu.edu.eg/studentLogin";
   let get_moodle_session_url: &str = "https://sis.eelu.edu.eg/getJCI";
   let mut sis = Sis::new(login_url, get_moodle_session_url, &headers_builder);

  // Login to sis
   match sis.login(&username, &password, UserType::Student).await {
        Ok(_) => {
            println!("Login Success");
           // Get moodle session link
          let Ok(moodle_session_link) = sis.get_moodle_session_link().await else { panic!("Failed to get moodle session link") };
          println!("Moodle session link: {}", moodle_session_link);
       },
        Err(err) => println!("Login Failed: {}", err),
    }
}
```
## Features
* `debug` - Enable debug logs, you still need to use a logger like env_logger and initialize it in your code

License: MIT OR Apache-2.0
