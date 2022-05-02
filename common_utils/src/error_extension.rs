

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, ErrorExtensions, FieldError, FieldResult, Object, ResultExt,
    Schema,
};


#[derive(Debug, Error)]
pub enum ServerError { 
    #[error("Could not find resource")]
    NotFound,

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}
impl ErrorExtensions for ServerError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            ServerError::NotFound => e.set("code", "NOT_FOUND"),
            ServerError::ServerError(reason) => e.set("reason", reason.to_string()),
            ServerError::ErrorWithoutExtensions => {}
        })
    }
}