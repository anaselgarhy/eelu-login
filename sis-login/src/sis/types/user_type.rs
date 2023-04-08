/// The type of user of the sis system
#[derive(Clone)]
pub enum UserType {
    /// A system user
    SystemUser,
    /// A staff member
    Staff,
    /// A student
    Student,
}

impl UserType {
    /// Create a new UserType from a number
    ///
    /// # Arguments
    /// * `num` - The number to convert to UserType (1 for SystemUser, 2 for Student, 3 for Staff)
    ///
    /// # Example
    /// ```
    /// # use sis_login::sis::types::user_type::UserType;
    ///
    /// let user_type = UserType::from(1);
    ///
    /// assert_eq!(user_type, UserType::SystemUser);
    /// ```
    /// * Note: If the number is not 1, 2 or 3, it will return UserType::Student
    /// # Example
    /// ```
    /// # use sis_login::sis::types::user_type::UserType;
    ///
    /// let user_type = UserType::from(4);
    ///
    /// assert_eq!(user_type, UserType::Student);
    /// ```
    pub fn from(num: u8) -> Self {
        match num {
            3 => Self::Staff,
            2 => Self::Student,
            1 => Self::SystemUser,
            _ => Self::Student,
        }
    }

    /// Create a new UserType from a string
    ///
    /// # Arguments
    /// * `user_str` - The string to convert to UserType ("system-user" for SystemUser, "student" for Student, "staff" for Staff)
    ///
    /// # Example
    /// ```
    /// # use sis_login::sis::types::user_type::UserType;
    ///
    /// let user_type = UserType::from_string(&"system-user".to_string());
    ///
    /// assert_eq!(user_type, UserType::SystemUser);
    /// ```
    /// * Note: If the string is not "system-user", "student" or "staff", it will return UserType::Student
    /// # Example
    /// ```
    /// # use sis_login::sis::types::user_type::UserType;
    ///
    /// let user_type = UserType::from_string(&"test".to_string());
    ///
    /// assert_eq!(user_type, UserType::Student);
    /// ```
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

    /// Convert the UserType to a number
    ///
    /// # Example
    /// ```
    /// # use sis_login::sis::types::user_type::UserType;
    ///
    /// let user_type = UserType::Staff;
    ///
    /// assert_eq!(user_type.to_num(), 3);
    /// ```
    pub fn to_num(&self) -> u8 {
        match self {
            Self::Staff => 3,
            Self::Student => 2,
            Self::SystemUser => 1,
        }
    }
}
