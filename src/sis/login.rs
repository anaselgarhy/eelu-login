use super::super::utils::{client, cookie_parser};
use super::types::{
    sis_response::{LoginResult, MoodleLoginResult},
    user_type::UserType,
};
use reqwest::{header::HeaderMap, Response};

pub async fn sis_login(username: &String, password: &String, usertype: UserType) -> Option<String> {
    let login_url: &str = "https://sis.eelu.edu.eg/studentLogin";
    let headers: HeaderMap = client::sis_eelu_request_headers(Some(&String::new()));
    let data: String = format!(
        "UserName={}&Password={}&userType={}",
        username,
        password,
        usertype.to_num()
    );
    let response: Response;

    println!(
        "Trying Login With => Username : {} , Password : {}  , As {}",
        username,
        password,
        usertype.to_string()
    );

    match client::get_client()
        .post(login_url)
        .body(data)
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => response = res,
        Err(err) => {
            println!("[-] Error While login : {}", err);
            return None;
        }
    };

    let res_headers = &response.headers().clone();

    let login_result: LoginResult;

    match response.json::<LoginResult>().await {
        Ok(result) => login_result = result,
        Err(err) => {
            println!("[-] Error While Parse Login Result : {}", err);
            return None;
        }
    }

    if login_result.rows[0].row.LoginOK.as_str() == "True" {
        println!("[+] Login Success");
        println!("[+] Getteing Session Moodle URL ...");
        return Some(cookie_parser::parse_cookies(&res_headers));
    } else {
        return None;
    }
}

pub async fn get_moodle_session(cookie: String) -> Option<String> {
    let url: &str = "https://sis.eelu.edu.eg/getJCI";
    let headers: HeaderMap = client::sis_eelu_request_headers(Some(&cookie));
    let data: &str = "param0=stuAdmission.stuAdmission&param1=moodleLogin&param2=2";
    let response: Response;

    match client::get_client()
        .post(url)
        .body(data)
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => response = res,
        Err(err) => {
            println!("[-] Error While Getteing Moodle URL : {}", err);
            return None;
        }
    };

    match response.json::<MoodleLoginResult>().await {
        Ok(result) => return Some(result.loginurl),
        Err(err) => {
            println!("[-] Error While Parse Login Result : {}", err);
            None
        }
    }
}

pub async fn moodle_login(
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
}
