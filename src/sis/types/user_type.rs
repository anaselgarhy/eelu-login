pub enum UserType{
    SystemUser,
    Staff,
    Student
}

impl UserType{

    pub fn from(num:u8)->Self{
           match num{
            3=> return Self::Staff,
            2=> return Self::Student,
            1=> return Self::SystemUser,
            _=> return Self::Student
        };
    }
    pub fn from_string(user_str:&String)->Self{
        match  user_str.as_str(){
           "staff"=> return Self::Staff,
           "student"=> return Self::Student,
           "system-user" =>return Self::SystemUser,
           _=> return Self::Student
        }
    }

    pub fn to_string(&self)->&str{
        match self{
            Self::Staff=>return "staff",
            Self::Student=>return "student",
            Self::SystemUser=>return "system-user"
        }
    }

    pub fn to_num(&self)->u8{
        match self{
            Self::Staff=>return 3,
            Self::Student=>return 2,
            Self::SystemUser=>return 1
        }
    }
}
