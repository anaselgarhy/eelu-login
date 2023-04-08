#[derive(serde::Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct LoginResult {
    pub rows: Vec<LoginRow>,
}
#[derive(serde::Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct LoginRow {
    pub row: LoginRowData,
}

#[derive(serde::Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct LoginRowData {
    #[serde(rename = "LoginOK")]
    pub login_ok: String,
}

#[derive(serde::Deserialize)]
pub struct MoodleLoginResult {
    #[serde(rename = "loginurl")]
    pub login_url: String,
}
