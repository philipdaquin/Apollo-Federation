use crate::schema::users;
use chrono::prelude::*;

#[derive(Clone, Debug, Identifiable, Queryable, PartialEq)]
#[table_name = "users"]
pub struct User { 
    pub id: i32, 
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub joined_at: NaiveDateTime
    
} 

#[derive(Clone, Debug, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser { 
    pub first_name: String,
    pub last_name: String,
    pub username: String, 
    pub password: String,
    pub email: String,
}

