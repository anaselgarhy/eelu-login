use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, COOKIE, REFERER, USER_AGENT},
    Client, ClientBuilder,
};

pub fn get_client() -> Client {
    match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => return client,
        Err(err) => panic!("[-] Error While Create client : {}", err),
    }
}

pub fn sis_eelu_request_headers(cookie: Option<&String>) -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("0xCrypt00o - EELU Moodle Login"),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
    );
    headers.insert(
        HeaderName::from_static("x-requested-with"),
        HeaderValue::from_static("XMLHttpRequest"),
    );
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://sis.eelu.edu.eg/static/PortalStudent.html"),
    );
    match cookie {
        Some(value) => {
            let boxed_cookie_str: Box<str> = value.clone().into_boxed_str();
            let cookie_str: &str = Box::leak(boxed_cookie_str);
            headers.insert(COOKIE, HeaderValue::from_static(cookie_str))
        }
        None => None,
    };
    return headers;
}
