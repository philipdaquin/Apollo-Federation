use chrono::NaiveDateTime;

pub struct RegisterUser;
pub mod register_user_mutation {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "RegisterUser";
    pub const QUERY : & str = "
        mutation RegisterUser($newUser: NewUserInput!) {\n  
            registerUser(newUser: $newUser) {\n  
            }\n
        }
    ";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[allow(dead_code)]
    type Date = NaiveDateTime;

    #[derive(Serialize)]
    pub struct NewUser {
        pub username: String, 
        pub first_name: String,
        pub last_name: String, 
        pub email: String,
        pub role: Option<String>
    }
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "newUser")]
        pub new_character: NewUser,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "registerUser")]
        pub new_character: Boolean,
    }
}
impl graphql_client::GraphQLQuery for RegisterUser {
    type Variables = register_user_mutation::Variables;
    type ResponseData = register_user_mutation::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: register_user_mutation::QUERY,
            operation_name: register_user_mutation::OPERATION_NAME,
        }
    }
}
