use async_graphql::*;
use async_graphql_actix_web::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use super::{resolver::{get_user_by_id, get_all_users, self}, model::User};
use crate::graphql::config::get_conn_from_ctx;


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
#[derive(Default)]
pub struct UserQuery;


#[Object(extends)]
impl UserQuery { 
    #[graphql(name = "getAllUsers")]
    async fn get_all(&self, ctx: &Context<'_>) -> Vec<UserType> { 
        resolver::get_all_users(&get_conn_from_ctx(ctx))
            .expect("Cannot Access Users from Database")
            .iter()
            .map(UserType::from)
            .collect()
    }
    pub async fn get_user_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<UserType> { 
        find_user_internally(ctx, id)
    }
    /// type User @key(fields: "id") { 
    ///     id: ID!
    ///     name: String
    ///     username: String
    /// }
    /// Resolver reference: the @key directive effectively tells the gateway, 
    /// This subgraph can resolve an instance of this entity if you provide its primary key 
    #[graphql(entity)]
    pub async fn find_user_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<UserType> { 
        find_user_internally(ctx, id)
    }
}
fn find_user_internally(ctx: &Context<'_>, id: ID) -> Option<UserType> {
    let id = id.parse::<i32>().expect("Failed To Parse from String");
    resolver::get_user_by_id(id, &get_conn_from_ctx(ctx))
        .ok()
        .map(|user| UserType::from(&user))
}


impl From<&User> for UserType { 
    fn from(user: &User) -> Self {
        Self { 
            id: user.id.into(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            username: user.username.clone(),
            password: user.password.clone(),
            email: user.email.clone(),
            joined_at: user.joined_at
        }
    }
}