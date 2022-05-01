use crate::schema::users;
use async_graphql::Enum;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};



#[derive(Clone, Debug, Identifiable, Queryable, PartialEq)]
#[table_name = "users"]
pub struct User { 
    pub id: i32, 
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub joined_at: NaiveDateTime,
    pub role: String,

    
} 
#[derive(Clone, Debug, Insertable, AsChangeset, PartialEq)]
#[table_name = "users"]
pub struct NewUser { 
    pub first_name: String,
    pub last_name: String,
    pub username: String, 
    pub password: String,
    pub email: String,
    pub role: String, 
}
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role { 
    Admin, 
    Customer,
    Operator,
    User
}