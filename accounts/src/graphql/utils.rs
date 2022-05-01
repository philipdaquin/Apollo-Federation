use actix_web::{Result, guard::Guard};
use argonautica::{Error, Hasher, Verifier};
use async_graphql::async_trait;
use lazy_static::lazy_static;
use async_graphql::*;

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

