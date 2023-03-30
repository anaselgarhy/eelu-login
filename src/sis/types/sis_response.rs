#[derive(serde::Deserialize)]
pub struct LoginResult{
    pub rows:Vec<LoginRow>
}
#[derive(serde::Deserialize)]
pub struct LoginRow{
pub row:LoginRowData
}

#[derive(serde::Deserialize)]
pub struct LoginRowData{
    pub LoginOK:String
}

#[derive(serde::Deserialize)]
pub struct MoodleLoginResult{
    pub loginurl:String
}
