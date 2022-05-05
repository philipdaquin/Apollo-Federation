
pub struct UserQuery;
pub mod user_query { 
    use super::*;
    use chrono::NaiveDateTime;
    pub const OPERATION_NAME: &str = "UserQuery";
    pub const QUERY: & str = "
        query  {
            \n  getAllUsers {
                \n    
                id\n    
                email\n    
                firstName\n    
                lastName\n    
                username\n    
                password\n    
                joinedAt\n  }
            \n
        }
        \n
        \n";
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

pub struct GetAllProducts;
pub mod product_query { 
    use super::*;
    use chrono::NaiveDateTime;
    pub const OPERATION_NAME: &str = "GetAllProducts";
    pub const QUERY: & str = "
        query  {
            \n  getAllProducts {
                \n    
                id\n    
                name\n    
                price\n    
                weight\n    
                category\n    
                createdBy\n
                tags\n    
                createdAt\n    
                updatedAt\n    
                description\n    
                imageUrl\n  }
            \n
        }
        \n
        \n";
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
        pub users: Vec<AllProductQuery>
    }
    #[derive(Deserialize)]
    pub struct AllProductQuery { 
        pub id: ID, 
        pub name: String,
        pub price: Option<i32>, 
        pub weight: Option<Int>,
        pub category: Option<String>, 
        pub created_by: Option<Int>,
        pub tags: Option<String>,
        pub created_at: Option<Date>,
        pub updated_at: Option<Date>,
        pub description: Option<String>,
        pub image_url: Option<String>
    }
}
impl graphql_client::GraphQLQuery for GetAllProducts { 
    type Variables = product_query::Variables;
    type ResponseData = product_query::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        use product_query::*;
        graphql_client::QueryBody { 
            variables,
            query:  QUERY,
            operation_name: OPERATION_NAME
        }
    }
}

pub struct GetProductById;
pub mod product_query_by_id { 
    use super::*;
    use chrono::NaiveDateTime;
    pub const OPERATION_NAME: &str = "GetProductById";
    pub const QUERY: & str = "
        query {
            \n  getProductById(id: $getProductByIdId) {
                \n    
                id\n    
                name\n    
                price\n    
                weight\n    
                category\n    
                createdBy\n
                tags\n    
                createdAt\n    
                updatedAt\n    
                description\n    
                imageUrl\n  }
            \n
        }
        \n
        \n";
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
    pub struct Variables { 
        #[serde(rename = "getProductByIdId")]
        product_id: ID
    }

    impl Variables {}
    /// The Response in the Accounts Service
    #[derive(Deserialize)]
    pub struct ResponseData { 
        #[serde(rename = "getProductById")]
        pub product_id: ProductQueryById
    }
    #[derive(Deserialize)]
    pub struct ProductQueryById { 
        pub id: ID, 
        pub name: String,
        pub price: Option<i32>, 
        pub weight: Option<Int>,
        pub category: Option<String>, 
        pub created_by: Option<Int>,
        pub tags: Option<String>,
        pub created_at: Option<Date>,
        pub updated_at: Option<Date>,
        pub description: Option<String>,
        pub image_url: Option<String>
    }
}
impl graphql_client::GraphQLQuery for GetProductById { 
    type Variables = product_query_by_id::Variables;
    type ResponseData = product_query_by_id::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        use product_query_by_id::*;
        graphql_client::QueryBody { 
            variables,
            query:  QUERY,
            operation_name: OPERATION_NAME
        }
    }
}


pub struct GetProductsByCategory;
pub mod product_query_by_category { 
    use super::*;
    use chrono::NaiveDateTime;
    pub const OPERATION_NAME: &str = "GetProductsByCategory";
    pub const QUERY: & str = "
        query {
            \n  getProductsByCategory(id: $category) {
                \n    
                id\n    
                name\n    
                price\n    
                weight\n    
                category\n    
                createdBy\n
                tags\n    
                createdAt\n    
                updatedAt\n    
                description\n    
                imageUrl\n  }
            \n
        }
        \n
        \n";
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
    pub struct Variables { 
        #[serde(rename = "category")]
        category: String
    }

    impl Variables {}
    /// The Response in the Accounts Service
    #[derive(Deserialize)]
    pub struct ResponseData { 
        #[serde(rename = "getProductsByCategory")]
        pub product_id: Vec<ProductQueryByCategory>
    }
    #[derive(Deserialize)]
    pub struct ProductQueryByCategory { 
        pub id: ID, 
        pub name: String,
        pub price: Option<i32>, 
        pub weight: Option<Int>,
        pub category: Option<String>, 
        pub created_by: Option<Int>,
        pub tags: Option<String>,
        pub created_at: Option<Date>,
        pub updated_at: Option<Date>,
        pub description: Option<String>,
        pub image_url: Option<String>
    }
}
impl graphql_client::GraphQLQuery for GetProductsByCategory { 
    type Variables = product_query_by_category::Variables;
    type ResponseData = product_query_by_category::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        use product_query_by_category::*;
        graphql_client::QueryBody { 
            variables,
            query:  QUERY,
            operation_name: OPERATION_NAME
        }
    }
}

pub struct GetProductsByTags;
pub mod product_query_by_tags { 
    use super::*;
    use chrono::NaiveDateTime;
    pub const OPERATION_NAME: &str = "GetProductsByTags";
    pub const QUERY: & str = "
        query {
            \n  getProductsByTags(tag: $tag) {
                \n    
                id\n    
                name\n    
                price\n    
                weight\n    
                category\n    
                createdBy\n
                tags\n    
                createdAt\n    
                updatedAt\n    
                description\n    
                imageUrl\n  
            }\n
        }
        \n
    \n";
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
    pub struct Variables { 
        #[serde(rename = "tag")]
        category: String
    }

    impl Variables {}
    /// The Response in the Accounts Service
    #[derive(Deserialize)]
    pub struct ResponseData { 
        #[serde(rename = "getProductsByTags")]
        pub product_id: Vec<ProductQueryByTags>
    }
    #[derive(Deserialize)]
    pub struct ProductQueryByTags { 
        pub id: ID, 
        pub name: String,
        pub price: Option<i32>, 
        pub weight: Option<Int>,
        pub category: Option<String>, 
        pub created_by: Option<Int>,
        pub tags: Option<String>,
        pub created_at: Option<Date>,
        pub updated_at: Option<Date>,
        pub description: Option<String>,
        pub image_url: Option<String>
    }
}
impl graphql_client::GraphQLQuery for GetProductsByTags { 
    type Variables = product_query_by_tags::Variables;
    type ResponseData = product_query_by_tags::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        use product_query_by_tags::*;
        graphql_client::QueryBody { 
            variables,
            query:  QUERY,
            operation_name: OPERATION_NAME
        }
    }
}

pub struct GetReviewsOfAnyProductId;
pub mod product_query_with_reviews { 
    use super::*;
    use chrono::NaiveDateTime;
    pub const OPERATION_NAME: &str = "GetReviewsOfAnyProductId";
    pub const QUERY: & str = "
        query GetReviewsOfAnyProductId($productId: ID!) {\n  
            getReviewsOfAnyProductId(productId: $productId) {\n    
                id\n    
                body\n    
                author {\n      
                    id\n      
                    username\n    
                }\n    
                heading\n    
                media\n    
                isEdited\n    
                userRating\n  
            }\n
         } 
    ";
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
    pub struct Variables { 
        #[serde(rename = "productId")]
        product_id: String,
    }

    impl Variables {}
    /// The Response in the Accounts Service
    #[derive(Deserialize)]
    pub struct ResponseData { 
        #[serde(rename = "getReviewsOfAnyProductId")]
        pub product: Vec<ReviewsOfProduct>,

    }
    #[derive(Deserialize)]
    pub struct ReviewsOfProduct { 
        pub id: ID, 
        pub name: String,
        pub price: Option<i32>, 
        pub weight: Option<Int>,
        pub category: Option<String>, 
        pub created_by: Option<Int>,
        pub tags: Option<String>,
        pub created_at: Option<Date>,
        pub updated_at: Option<Date>,
        pub description: Option<String>,
        pub image_url: Option<String>
    }


}
impl graphql_client::GraphQLQuery for GetReviewsOfAnyProductId { 
    type Variables = product_query_with_reviews::Variables;
    type ResponseData = product_query_with_reviews::ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        use product_query_with_reviews::*;
        graphql_client::QueryBody { 
            variables,
            query:  QUERY,
            operation_name: OPERATION_NAME
        }
    }
}


/// ""