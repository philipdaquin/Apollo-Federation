use async_graphql::*;
use async_graphql_actix_web::*;
use chrono::NaiveDateTime;


#[derive(SimpleObject)]
pub struct UserType { 
    pub id: ID,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub joined_at: NaiveDateTime
}