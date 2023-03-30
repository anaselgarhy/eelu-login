use reqwest::header::HeaderMap;

fn get_cookie(res_cookie_header: &str) -> &str {
    let res_cookie_header_bytes: &[u8] = res_cookie_header.as_bytes();
    for (index, value) in res_cookie_header_bytes.iter().enumerate() {
        if *value as char == ';' {
            if index + 1 >= res_cookie_header_bytes.len() {
                return &res_cookie_header[..index + 1];
            } else if res_cookie_header_bytes[index + 1] as char == ' ' {
                return &res_cookie_header[..index + 1];
            }
        }
    }
    &res_cookie_header[..]
}

pub fn parse_cookies(headers: &HeaderMap) -> String {
    let mut cookie: String = String::new();
    for (_key, value) in headers.iter().enumerate() {
        if value.0.as_str() == "set-cookie" {
            if cookie.is_empty() {
                cookie = format!("{}", get_cookie(value.1.to_str().unwrap()));
            } else {
                cookie = format!("{} {}", cookie, get_cookie(value.1.to_str().unwrap()));
            }
        }
    }
    return cookie;
}
