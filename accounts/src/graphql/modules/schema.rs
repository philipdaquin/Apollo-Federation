use std::str::FromStr;

use async_graphql::*;
use async_graphql_actix_web::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use redis::{aio::ConnectionManager, Value,  AsyncCommands, RedisError};
use serde::{Serialize, Deserialize};
use super::{resolver::{get_user_by_id, get_all_users, self}, 
    model::{User, NewUser, Role}
};
use common_utils::{Role as AuthenticationRole};
use crate::{graphql::{utils::RoleGuard, config::{get_redis_conn_manager, get_redis_conn_from_ctx}}, redis::{get_post_cache_key, create_connection}};
use async_graphql::Guard;
use crate::graphql::{config::get_conn_from_ctx, utils::verify_password};
use common_utils::*;
use jsonwebtoken;
use crate::graphql::utils::*;

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct UserType { 
    pub id: ID,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub joined_at: NaiveDateTime,
    pub role: String
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
    #[graphql(name = "getUserByUserId")]
    pub async fn get_user_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<UserType> { 
        let cache_key = get_post_cache_key(id.to_string().as_str());
        let redis_client = get_redis_conn_from_ctx(ctx).await;
        let mut redis_connection = create_connection(redis_client)
            .await
            .expect("Unable to create Redis DB connection");
        let cached_object = redis_connection.get(cache_key.clone()).await.expect("Redis Error on Client ");
        
            //  Check If Cache Object is available 
        match cached_object { 
            Value::Nil => { 
                log::info!("Unable to find cache under this id, accessing Database.. 😂");

                let user = find_user_internally(ctx, id);
                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, user.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut redis_connection)
                    .await
                    .expect("Internal Error Occurred while attempting to cache the object");
                return user
            },
            Value::Data(cache) => { 
                log::info!("Cache Found Under this Id! 👌");
                serde_json::from_slice(&cache).expect("Unable to Deserialize Struct")
            },
            _ => { None }
        }
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


#[derive(Default)]
pub struct UserMutation;
#[derive(InputObject)]
pub struct NewUserInput { 
    pub first_name: String,
    pub last_name: String,
    pub username: String, 
    pub password: String,
    pub email: String,
    pub role: Option<Role>
}
#[derive(InputObject)] 
pub struct UserLogin { 
    pub username: String, 
    pub password: String
}

#[Object]
impl UserMutation { 
    /// Delete a User using User ID
    #[graphql(name = "deleteUserInDatabase")]   
    pub async fn delete_user(&self, ctx: &Context<'_>, user_id: ID) -> FieldResult<bool> { 
        resolver::delete_user(parse_id(user_id.clone()), &get_conn_from_ctx(ctx))?;

        // Delete Cache under this id
        let cache_id = get_post_cache_key(user_id.to_string().as_str());
        get_redis_conn_manager(ctx)
            .await
            .del(cache_id)
            .await?;

        Ok(true)
    }

    #[graphql(name = "registerUser")]
    pub async fn register_user(&self, ctx: &Context<'_>, new_user: NewUserInput) ->  UserType { 
        let user = resolver::create_user(
            NewUser::from(&new_user), &get_conn_from_ctx(ctx))
            .expect("Unable to Convert 'NewUserInput' type to 'NewUser'");
        UserType::from(&user)
    }
    
    #[graphql(name = "updateUserDetails")]
    pub async fn update_user(&self, ctx: &Context<'_>, new_user: NewUserInput, id: ID) -> FieldResult<UserType> { 
        let updated_user = resolver::update_user_details(
            parse_id(id.clone()), 
            NewUser::from(&new_user), 
            &get_conn_from_ctx(ctx)
        ).expect("Panic at Updating the User Details");

        //  Delete the cache under this value 
        let cache_key = get_post_cache_key(id.to_string().as_str());
        let _: () = get_redis_conn_manager(ctx)
            .await
            .del(cache_key)
            .await?;

        Ok(UserType::from(&updated_user))
    } 

    #[graphql(name = "loginUser")]
    pub async fn login_user(&self, ctx: &Context<'_>, login: UserLogin) -> Result<String, Error> { 
        // Get user by username 
        let user_info = resolver::get_user_by_name(login.username, &get_conn_from_ctx(ctx)).ok();
        if let Some(info) = user_info { 
 
            if let Ok(role) = verify_password(&info.password, &login.password)  {
                if role { 
                    let id = AuthenticationRole::from_str(info.role.as_str())
                        .expect("");
                    let token =  generate_token(info.username, id);
                    return Ok(token)
                }
            }
        }
        Err(Error::new("Probably Wrong Password? 🤣"))
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard  {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> { 
        
        if ctx.data_opt::<AuthenticationRole>() == Some(&AuthenticationRole::Admin) 
        || ctx.data_opt::<AuthenticationRole>() == Some(&self.role) {
            Ok(())
        } else {
            let guard_error = ctx.data_opt::<jsonwebtoken::errors::Error>().clone();
            match guard_error {
                Some(e) => return Err(format!("{:?}", e.kind()).into()),
                None => return Err(format!("Access denied: {} role required", &self.role).into())
            }
        }
    }
}
//  Helper Parser 
pub fn parse_id(id: ID) -> i32 { 
    id.parse::<i32>().expect("ParseIntError")
}