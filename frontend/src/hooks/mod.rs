pub mod use_query;
use graphql_client::GraphQLQuery;
use reqwest::{Error, Response};
use serde::Deserialize;
use serde_json::Value;


#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

pub async fn build_request(request_json: &Value) -> Result<Response, Error> {
    let api_url = option_env!("API_URL").unwrap_or("http://127.0.0.1:8000/graphql");
    let response = reqwest::Client::new()
        .post(api_url)
        .json(request_json)
        .send()
        .await;

    response
}