use std::str::FromStr;

use crate::{schema::users, graphql::utils::hash_password};
use async_graphql::Enum;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};

use super::schema::{UserType, NewUserInput};



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

/// Diesel Type into Graphql typ e
impl From<&User> for UserType { 
    fn from(user: &User) -> Self {
        Self { 
            id: user.id.into(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            username: user.username.clone(),
            password: user.password.clone(),
            email: user.email.clone(),
            joined_at: user.joined_at,
            role: Role::from_str(user.role.as_str())
                .unwrap_or(Role::User).to_string()
        }
    }
}
/// Convert Graphql Type into Reading Database Type 
impl From<&NewUserInput> for NewUser { 
    fn from(f: &NewUserInput) -> Self {
        Self { 
            first_name: f.first_name.clone(),
            last_name: f.last_name.clone(), 
            username: f.username.clone(),
            password: hash_password(f.password.as_str())
                .expect("Can't get the hash for password"), 
            email: f.email.clone(),
            role: f.role.unwrap_or(Role::User).to_string()
        }
    }
}