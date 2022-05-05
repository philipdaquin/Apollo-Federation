pub mod use_query;
use graphql_client::GraphQLQuery;
use reqwest::{Error, Response};
use serde::Deserialize;
use serde_json::Value;
use chrono::NaiveDateTime;


pub use use_query::use_query;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Clone, PartialEq"
)]
pub struct UserQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Clone, PartialEq"
)]
pub struct GetAllProducts;

/// Get Products ID
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Clone, PartialEq"
)]
pub struct GetProductById;

///
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Clone, PartialEq"
)]
pub struct GetProductsByCategory;



////
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Clone, PartialEq")]
pub struct GetProductsByTags;




#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql",
    response_derives = "Clone, PartialEq")]
pub struct GetProductByIdWithReviews;





////
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

pub async fn build_request(request_json: &Value) -> Result<Response, Error> {
    let api_url = option_env!("API_URL").unwrap_or("http://localhost:4000/graphqll");
    let response = reqwest::Client::new()
        .post(api_url)
        .json(request_json)
        .send()
        .await;

    response
}