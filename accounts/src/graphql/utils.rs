use actix_web::{Result};
use argonautica::{Error, Hasher, Verifier};
use async_graphql::async_trait;
use lazy_static::lazy_static;
use async_graphql::*;
use async_graphql::Guard;
use super::modules::model::Role;

lazy_static! {
    static ref PASSWORD_SECRET_KEY: String =
        std::env::var("PASSWORD_SECRET_KEY").expect("Can't read PASSWORD_SECRET_KEY");
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .hash()
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .verify()
}

use common_utils::Role as AuthenticationRole;
pub struct RoleGuard { 
    pub role: AuthenticationRole
}
impl RoleGuard { 
    fn new(role: AuthenticationRole) -> Self { 
        RoleGuard { 
            role
        }
    }
}



/// Field will be visible to users with Role::Admin and
/// Role::Analyst
pub fn is_customer(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<Role>() == Some(&Role::Admin) ||
    ctx.data_opt::<Role>() == Some(&Role::Customer)
}

/// Field will be visible to users with Role::Admin and
/// Role::Analyst
pub fn is_operator(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<Role>() == Some(&Role::Admin) ||
    ctx.data_opt::<Role>() == Some(&Role::Operator)
}

/// Field will only be visible to users with Role::Admin
pub fn is_admin(ctx: &Context<'_>) -> bool {
    ctx.data_opt::<Role>() == Some(&Role::Admin)
}