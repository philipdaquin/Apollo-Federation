use graphql_client::{GraphQLQuery, Response};
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{use_effect_with_deps, use_state};

use super::build_request;

#[derive(Clone)]
pub struct QueryResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub loading: bool,
}

pub fn use_query<Q>(variables: Q::Variables) -> QueryResponse<Q::ResponseData>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + 'static,
{
    let state = use_state(|| QueryResponse {
        data: None,
        error: None,
        loading: true,
    });

    let effect_state = state.clone();

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let request_body = Q::build_query(variables);
                let request_json = &json!(request_body);
                let request = build_request(request_json).await;
                match request {
                    Ok(response) => {
                        let json = response.json::<Response<Q::ResponseData>>().await;
                        match json {
                            Ok(response) => effect_state.set(QueryResponse {
                                data: response.data,
                                error: None,
                                loading: false,
                            }),
                            Err(error) => effect_state.set(QueryResponse {
                                data: None,
                                error: Some(error.to_string()),
                                loading: false,
                            }),
                        }
                    }
                    Err(error) => effect_state.set(QueryResponse {
                        data: None,
                        error: Some(error.to_string()),
                        loading: false,
                    }),
                }
            });

            || ()
        },
        (),
    );

    (*state).clone()
}