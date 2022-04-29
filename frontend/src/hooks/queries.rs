
pub struct UserQuery;
use chrono::NaiveDateTime;


pub mod user_query { 
    use super::*;
    use chrono::NaiveDateTime;
    pub const OPERATION_NAME: &str = "UserQuery";
    pub const QUERY: & str = "
        query {
            \n  getAllUsers {
                \n    
                id\n    
                email\n    
                firstName\n    
                lastName\n    
                username\n    
                password\n    
                joinedAt\n  
            }\n
        }\n\n";
    use serde::{Deserialize, Serialize};

    #[allow(dead_code)]
    type Boolean = bool;

    #[allow(dead_code)]
    type Float = f64;
    
    #[allow(dead_code)]
    type Int = i32;
    
    #[allow(dead_code)]
    type ID = String;

    type Date = NaiveDateTime;

    /// No Variables in getAllUsers
    #[derive(Serialize)]
    pub struct Variables;
    
    /// The Response in the Accounts Service
    #[derive(Deserialize)]
    pub struct ResponseData { 
        pub users: Vec<AllUserQuery>
    }
    #[derive(Deserialize)]
    pub struct AllUserQuery { 
        pub id: ID, 
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub password: String,
        pub email: String,
        pub joined_at: Date
    }
}
impl graphql_client::GraphQLQuery for UserQuery { 
    type Variables = user_query::Variables;
    type ResponseData = user_query::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        use user_query::*;
        graphql_client::QueryBody { 
            variables,
            query:  QUERY,
            operation_name: OPERATION_NAME
        }
    }

}
