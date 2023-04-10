//! A library to login to the sis system and get the moodle session
//!
//! # Example
//! ```ignore
//! use sis_login::Sis;
//! use sis_login::sis::types::user_type::UserType;
//!
//! #[tokio::main]
//! async fn main() {
//!    let username = std::env::var("SIS_USERNAME").unwrap();
//!    let password = std::env::var("SIS_PASSWORD").unwrap();
//!
//!    // Crate Sis instance
//!    let headers_builder = sis_login::headers_builder::DefaultHeadersBuilder::new(
//!       "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0".to_string(),
//!      "https://sis.eelu.edu.eg/static/PortalStudent.html".to_string()
//!   );
//!
//!    let login_url: &str = "https://sis.eelu.edu.eg/studentLogin";
//!    let get_moodle_session_url: &str = "https://sis.eelu.edu.eg/getJCI";
//!    let mut sis = Sis::new(login_url, get_moodle_session_url, &headers_builder);
//!
//!   // Login to sis
//!    match sis.login(&username, &password, UserType::Student).await {
//!         Ok(_) => {
//!             println!("Login Success");
//!            // Get moodle session link
//!           let Ok(moodle_session_link) = sis.get_moodle_session_link().await else { panic!("Failed to get moodle session link") };
//!           println!("Moodle session link: {}", moodle_session_link);
//!        },
//!         Err(err) => println!("Login Failed: {}", err),
//!     }
//! }
//!```
//! # Features
//! * `debug` - Enable debug logs, you still need to use a logger like env_logger and initialize it in your code

pub mod headers_builder;
pub mod sis;

use crate::sis::types::sis_response::{LoginResult, MoodleLoginResult};
use crate::sis::types::user_type::UserType;
use crate::sis::utils;
#[cfg(feature = "debug")]
use log::{debug, error, info};
use std::future::{IntoFuture, Ready};

/// The error type for the Sis struct
pub enum SisError {
    /// There was an error while sending the request to the server (It can be a network error or a server error)
    SendRequestError(reqwest::Error),
    /// There was an error creating the client that will be used to send the requests
    CreateClientError(reqwest::Error),
    /// The provided username or password is incorrect
    AuthError,
    /// There was an error while parsing the response from the server (Unexpected response)
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

/// This struct is used to login to the sis system and get the moodle session.
pub struct Sis<'a> {
    login_url: String,
    get_moodle_session_url: String,
    cookies: String,
    headers_builder: &'a (dyn headers_builder::HeadersBuilder + 'a),
}

impl<'a> Sis<'a> {
    /// Create a new Sis instance
    ///
    /// # Arguments
    /// * `login_url` - The login url of the sis system (It varies by university, for example in EELU it's https://sis.eelu.edu.eg/studentLogin)
    /// * `get_moodle_session_url` - The url to get the moodle session (It varies by university, for example in EELU it's https://sis.eelu.edu.eg/getJCI)
    /// * `headers_builder` - The headers builder to use (In most cases you can use the default one or you can create your own if you want more control)
    ///
    /// # Example
    /// ```!
    /// # use sis_login::Sis;
    /// # use sis_login::headers_builder::DefaultHeadersBuilder;
    /// # use sis_login::sis::types::user_type::UserType;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let headers_builder = DefaultHeadersBuilder::new(
    ///       "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0".to_string(),
    ///      "https://sis.eelu.edu.eg/static/PortalStudent.html".to_string(),
    ///   );
    ///  let mut sis = Sis::new(
    ///     "https://sis.eelu.edu.eg/studentLogin",
    ///    "https://sis.eelu.edu.eg/getJCI",
    ///     &headers_builder,
    ///  );
    ///
    ///  // Use the sis instance here...
    /// }
    pub fn new(
        login_url: &str,
        get_moodle_session_url: &str,
        headers_builder: &'a (dyn headers_builder::HeadersBuilder + 'a),
    ) -> Self {
        Sis {
            login_url: login_url.to_string(),
            get_moodle_session_url: get_moodle_session_url.to_string(),
            cookies: String::new(),
            headers_builder,
        }
    }

    /// Login to sis
    /// # Arguments
    /// * `username` - The username of the user
    /// * `password` - The password of the user
    /// * `usertype` - The type of the user (Student or Staff or System user)
    ///
    /// # Example
    /// ```ignore
    /// # use sis_login::Sis;
    /// # use sis_login::sis::types::user_type::UserType;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let username = std::env::var("SIS_USERNAME").unwrap();
    ///    let password = std::env::var("SIS_PASSWORD").unwrap();
    ///
    ///    // Crate Sis instance
    ///    let headers_builder = sis_login::headers_builder::DefaultHeadersBuilder::new(
    ///       "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0".to_string(),
    ///      "https://sis.eelu.edu.eg/static/PortalStudent.html".to_string()
    ///   );
    ///
    ///    let login_url: &str = "https://sis.eelu.edu.eg/studentLogin";
    ///    let get_moodle_session_url: &str = "https://sis.eelu.edu.eg/getJCI";
    ///    let mut sis = Sis::new(login_url, get_moodle_session_url, &headers_builder);
    ///
    ///   // Login to sis
    ///    match sis.login(&username, &password, UserType::Student).await {
    ///         Ok(_) => println!("Login Success"),
    ///         Err(err) => println!("Login Failed: {}", err),
    ///     }
    /// }
    ///```
    ///
    /// # Errors
    /// * `SisError::SendRequestError` - If there is an error while sending the request (e.g. network error)
    /// * `SisError::CreateClientError` - If there is an error while creating the client (e.g. invalid url)
    /// * `SisError::AuthError` - If the provided username or password is incorrect
    /// * `SisError::ParseLoginResultError` - If there is an error while parsing the login result
    pub async fn login(
        &mut self,
        username: &String,
        password: &String,
        usertype: UserType,
    ) -> Result<()> {
        // let login_url: &str = "https://sis.eelu.edu.eg/studentLogin";
        let data = format!(
            "UserName={}&Password={}&userType={}",
            username,
            password,
            usertype.to_num()
        );

        #[cfg(feature = "debug")]
        debug!(
            "Trying Login With => Username : {} , Password : {}  , As {}",
            username,
            password,
            usertype.to_string()
        );

        let response =
            utils::send_request(self.login_url.as_str(), data, self.headers_builder.build())
                .await?;

        let res_headers = &response.headers().clone();

        #[cfg(feature = "debug")]
        debug!("Response Headers: {:?}", res_headers);

        let login_result = match response.json::<LoginResult>().await {
            Ok(result) => result,
            Err(err) => {
                #[cfg(feature = "debug")]
                debug!("[-] Error While Parse Login Result : {}", err);
                return Err(SisError::ParseLoginResultError);
            }
        };

        #[cfg(feature = "debug")]
        debug!("Login Result: {:?}", login_result);

        if login_result.rows[0].row.login_ok.as_str() == "True" {
            #[cfg(feature = "debug")]
            {
                info!("[+] Login Success");
                info!("[+] Getteing Session Moodle URL ...");
            }
            self.cookies = utils::parse_cookies(res_headers);
            Ok(())
        } else {
            Err(SisError::AuthError)
        }
    }

    /// Get Moodle Session URL
    ///
    /// # Example
    /// ```ignore
    /// # use sis_login::Sis;
    /// # use sis_login::sis::types::user_type::UserType;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let username = std::env::var("SIS_USERNAME").unwrap();
    ///     let password = std::env::var("SIS_PASSWORD").unwrap();
    ///
    ///     // Crate Sis instance
    ///     let headers_builder = sis_login::headers_builder::DefaultHeadersBuilder::new(
    ///         "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0".to_string(),
    ///         "https://sis.eelu.edu.eg/static/PortalStudent.html".to_string()
    ///     );
    ///
    ///     let login_url: &str = "https://sis.eelu.edu.eg/studentLogin";
    ///     let get_moodle_session_url: &str = "https://sis.eelu.edu.eg/getJCI";
    ///     let mut sis = Sis::new(login_url, get_moodle_session_url, &headers_builder);
    ///
    ///     // Login to sis
    ///     match sis.login(&username, &password, UserType::Student).await {
    ///         Ok(_) => println!("Login Success"),
    ///         Err(err) => println!("Login Failed: {}", err),
    ///     }
    ///
    ///     // Get Moodle Session URL
    ///     match sis.get_moodle_session().await {
    ///         Ok(url) => println!("Moodle Session URL: {}", url),
    ///         Err(err) => println!("Error While Get Moodle Session URL: {}", err),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// * `SisError::SendRequestError` - If there is an error while sending the request (e.g. network error)
    /// * `SisError::CreateClientError` - If there is an error while creating the client (e.g. invalid url)
    /// * `SisError::ParseLoginResultError` - If there is an error while parsing the login result (e.g. invalid response)
    pub async fn get_moodle_session(&self) -> Result<String> {
        // let url: &str = "https://sis.eelu.edu.eg/getJCI";

        let response = utils::send_request(
            self.get_moodle_session_url.as_str(),
            "param0=stuAdmission.stuAdmission&param1=moodleLogin&param2=2".to_string(),
            self.headers_builder
                .build_with_cookies(self.cookies.as_str()),
        )
        .await?;

        match response.json::<MoodleLoginResult>().await {
            Ok(result) => Ok(result.login_url),
            Err(err) => {
                #[cfg(feature = "debug")]
                error!("[-] Error While Parse Login Result : {}", err);
                Err(SisError::ParseLoginResultError)
            }
        }
    }
}
