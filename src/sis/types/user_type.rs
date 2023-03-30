pub enum UserType{
    SystemUser,
    Admin,
    Student
}

impl UserType{

    pub fn from(num:u8)->Self{
           match num{
            0=> return Self::Admin,
            2=> return Self::Student,
            1=> return Self::SystemUser,
            _=> return Self::Student
        };
    }
    pub fn from_string(user_str:&String)->Self{
        match  user_str.as_str(){
           "admin"=> return Self::Admin,
           "student"=> return Self::Student,
           "system-user" =>return Self::SystemUser,
           _=> return Self::Student
        }
    }

    pub fn to_string(&self)->&str{
        match self{
            Self::Admin=>return "admin",
            Self::Student=>return "student",
            Self::SystemUser=>return "system-user"
        }
    }

    pub fn to_num(&self)->u8{
        match self{
            Self::Admin=>return 0,
            Self::Student=>return 2,
            Self::SystemUser=>return 1
        }
    }
}
