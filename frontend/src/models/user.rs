use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::hooks::user_query::UserQueryGetAllUsers;

#[derive(Clone, Debug)]
pub struct UserInfo { 
    pub id: i32, 
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub joined_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NewUserRegister { 
    pub username: String, 
    pub first_name: String,
    pub last_name: String, 
    pub email: String,
    pub role: Option<String>
}




impl From<&UserQueryGetAllUsers> for UserInfo { 
    fn from(user: &UserQueryGetAllUsers) -> Self {
        Self { 
            id: user.id.parse::<i32>().expect(""), 
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            username: user.last_name.clone(),
            password: user.password.clone(),
            email: user.email.clone(),
            joined_at: user.joined_at.clone()
        }
    }
} 