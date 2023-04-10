use rustls::Error::DecryptError;

#[derive(serde::Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct LoginResult {
    pub rows: Vec<LoginRow>,
}
#[derive(serde::Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct LoginRow {
    pub row: LoginRowData,
}

#[derive(serde::Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub(crate) struct LoginRowData {
    #[serde(rename = "LoginOK")]
    pub login_ok: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct MoodleLoginResult {
    #[serde(rename = "loginurl")]
    pub login_url: String,
}
