use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, COOKIE, REFERER, USER_AGENT,
};

pub trait HeadersBuilder {
    fn build(&self) -> HeaderMap;
    fn update_cookies(&self, headers: &mut HeaderMap, cookies: &str) {
        let boxed_cookie_str: Box<str> = cookies.to_string().into_boxed_str();
        let cookie_str: &str = Box::leak(boxed_cookie_str);
        headers.insert(COOKIE, HeaderValue::from_static(cookie_str));
    }
    fn build_with_cookies(&self, cookies: &str) -> HeaderMap {
        let mut headers = self.build();
        self.update_cookies(&mut headers, cookies);
        headers
    }
}

pub struct DefaultHeadersBuilder {
    user_agent: String,
    referer: String,
}

impl DefaultHeadersBuilder {
    pub fn new(user_agent: String, referer: String) -> Self {
        Self {
            user_agent,
            referer,
        }
    }
}

impl HeadersBuilder for DefaultHeadersBuilder {
    fn build(&self) -> HeaderMap {
        let mut headers: HeaderMap = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(Box::leak(self.user_agent.clone().into_boxed_str())),
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
            HeaderValue::from_static(Box::leak(self.referer.clone().into_boxed_str())),
        );

        headers
    }
}
