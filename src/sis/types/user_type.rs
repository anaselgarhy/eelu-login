#[derive(Clone)]
pub enum UserType {
    SystemUser,
    Staff,
    Student,
}

impl UserType {
    pub fn from(num: u8) -> Self {
        match num {
            3 => Self::Staff,
            2 => Self::Student,
            1 => Self::SystemUser,
            _ => Self::Student,
        }
    }
    pub fn from_string(user_str: &String) -> Self {
        match user_str.as_str() {
            "staff" => Self::Staff,
            "student" => Self::Student,
            "system-user" => Self::SystemUser,
            _ => Self::Student,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::Staff => "staff",
            Self::Student => "student",
            Self::SystemUser => "system-user",
        }
    }

    pub fn to_num(&self) -> u8 {
        match self {
            Self::Staff => 3,
            Self::Student => 2,
            Self::SystemUser => 1,
        }
    }
}
