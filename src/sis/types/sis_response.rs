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
    sysID:String,
    userID:String,
    password:String,
    currentIP:String,
    username:String,
    pub LoginOK:String
}

#[derive(serde::Deserialize)]
pub struct MoodleLoginResult{
    pub loginurl:String
}
