use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, COOKIE, REFERER, USER_AGENT,
};

/// This trait is used to build the headers for the requests.
/// The default implementation is `DefaultHeadersBuilder`.
/// You can implement this trait for your own headers builder.
///
/// # Example
/// ```
/// use reqwest::header::{HeaderMap, HeaderValue};
/// use sis_login::headers_builder::HeadersBuilder;
/// use sis_login::headers_builder::DefaultHeadersBuilder;
///
/// struct MyHeadersBuilder;
///
/// impl HeadersBuilder for MyHeadersBuilder {
///    fn build(&self) -> HeaderMap {
///       let mut headers: HeaderMap = HeaderMap::new();
///      headers.insert("my-header", HeaderValue::from_static("my-value"));
///      // at so on...
///     headers
///   }
/// }
/// ```
pub trait HeadersBuilder {
    /// This method is used to build the headers.
    /// It calls every time we send request doesn't need cookies.
    ///
    /// - see `build_with_cookies`
    fn build(&self) -> HeaderMap;

    /// This method is used to update the cookies in the headers.
    /// It calls every time we send request and need cookies.
    ///
    /// - see `build_with_cookies`
    fn update_cookies(&self, headers: &mut HeaderMap, cookies: &str) {
        let boxed_cookie_str: Box<str> = cookies.to_string().into_boxed_str();
        let cookie_str: &str = Box::leak(boxed_cookie_str);
        headers.insert(COOKIE, HeaderValue::from_static(cookie_str));
    }

    /// This method is used to build the headers.
    /// It calls every time we send request and need cookies.
    /// The default implementation is simply calls `build` and then calls `update_cookies`.
    ///
    /// - see `build`, `update_cookies`
    fn build_with_cookies(&self, cookies: &str) -> HeaderMap {
        let mut headers = self.build();
        self.update_cookies(&mut headers, cookies);
        headers
    }
}

/// The default implementation of `HeadersBuilder`.
/// It builds the headers with an custom user agent and referer.
pub struct DefaultHeadersBuilder {
    user_agent: String,
    referer: String,
}

impl DefaultHeadersBuilder {
    /// Creates a new `DefaultHeadersBuilder` with the given user agent and referer.
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
