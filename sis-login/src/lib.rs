pub mod sis;
pub mod headers_builder;


use std::future::{IntoFuture, Ready};
use std::process::Output;
use reqwest::{header::HeaderMap, Response};
#[cfg(feature = "debug")]
extern crate env_logger;
#[cfg(feature = "debug")]
#[macro_use]
extern crate log;
use crate::sis::types::sis_response::{LoginResult, MoodleLoginResult};
use crate::sis::types::user_type::UserType;
use crate::sis::utils;

pub enum SisError {
    SendRequestError(reqwest::Error),
    CreateClientError(reqwest::Error),
    /// The provided username or password is incorrect
    AuthError,
    ParseLoginResultError,
}

impl IntoFuture for SisError {
    type Output = SisError;
    type IntoFuture = Ready<Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        std::future::ready(self)
    }
}

/// A Result type alias for SisError
pub type Result<T> = std::result::Result<T, SisError>;

pub struct Sis<'a> {
    login_url: String,
    get_moodle_session_url: String,
    cookies: String,
    headers_builder: &'a (dyn headers_builder::HeadersBuilder + 'a),
}

impl<'a> Sis<'a> {
    /// Create a new Sis instance
    pub fn new(login_url: &str, get_moodle_session_url: &str, headers_builder: &'a (dyn headers_builder::HeadersBuilder + 'a)) -> Self {
        Sis {
            login_url: login_url.to_string(),
            get_moodle_session_url: get_moodle_session_url.to_string(),
            cookies: String::new(),
            headers_builder,
        }
    }
    pub async fn login(&mut self, username: &String, password: &String, usertype: UserType) -> Result<()> {
        // let login_url: &str = "https://sis.eelu.edu.eg/studentLogin";
        let data = format!(
            "UserName={}&Password={}&userType={}",
            username,
            password,
            usertype.to_num()
        );

        #[cfg(feature = "debug")]
        println!(
        "Trying Login With => Username : {} , Password : {}  , As {}",
        username,
        password,
        usertype.to_string()
    );

        let response = utils::send_request(self.login_url.as_str(), data, self.headers_builder.build()).await?;

        let res_headers = &response.headers().clone();

        #[cfg(feature = "debug")]
        println!("Response Headers: {:?}", res_headers);

        let login_result = match response.json::<LoginResult>().await {
            Ok(result) => result,
            Err(err) => {
                #[cfg(feature = "debug")]
                println!("[-] Error While Parse Login Result : {}", err);
                return Err(SisError::ParseLoginResultError);
            }
        };

        #[cfg(feature = "println!()")]
        println!("Login Result: {:?}", login_result);

        if login_result.rows[0].row.login_ok.as_str() == "True" {
            #[cfg(feature = "debug")] {
                info!("[+] Login Success");
                info!("[+] Getteing Session Moodle URL ...");
            }
            self.cookies = utils::parse_cookies(&res_headers);
            Ok(())
        } else {
            Err(SisError::AuthError)
        }
    }

    pub async fn get_moodle_session(&self) -> Result<String> {
        // let url: &str = "https://sis.eelu.edu.eg/getJCI";

        let response = utils::send_request(self.get_moodle_session_url.as_str(),
                                           "param0=stuAdmission.stuAdmission&param1=moodleLogin&param2=2".to_string(),
                                           self.headers_builder.build_with_cookies(self.cookies.as_str())).await?;

        match response.json::<MoodleLoginResult>().await {
            Ok(result) => Ok(result.login_url),
            Err(err) => {
                #[cfg(feature = "debug")]
                error!("[-] Error While Parse Login Result : {}", err);
                Err(SisError::ParseLoginResultError)
            }
        }
    }

    /*    pub async fn moodle_login(
            username: &String,
            password: &String,
            usertype: UserType,
        ) -> Option<String> {
            let cookie: Option<String> = sis_login(username, password, usertype).await;
            if cookie.is_some() {
                loop {
                    let moodle_session_url = get_moodle_session(cookie.clone().unwrap()).await;
                    if moodle_session_url.is_some() {
                        return moodle_session_url;
                    }
                }
            } else {
                None
            }
        }*/
}
