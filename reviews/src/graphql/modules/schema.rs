use async_graphql::*;
use async_graphql_actix_web::*;
use super::resolver::{get_reviews, get_reviews_by_product, get_reviews_by_user, self};
use crate::graphql::config::get_conn_from_ctx;

#[derive(SimpleObject)]
pub struct ReviewType { 
    pub id: ID,
    pub body: String,
    pub author: UserType,
    pub product: ProductType 
}

pub struct UserType { 
    /// external
    pub id: ID
}
#[Object(extends)]
impl UserType { 
    #[graphql(external)]
    pub async fn id(&self) -> &ID { 
        &self.id
    }
    #[graphql(name = "getUserReviews")]
    pub async fn reviews(&self, ctx: &Context<'_>, id: ID) -> Vec<ReviewType> { 
        let id = id.parse::<i32>().expect("");
        resolver::get_reviews_by_user(id, &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|x| ReviewType::from(x))
            .collect()
    }
}
pub struct ProductType { 
    pub id: ID
}
#[Object(extends)]
impl ProductType { 
    #[graphql(external)]
    pub async fn id(&self) -> &ID { 
        &self.id
    }    
    #[graphql(name = "getProductReviews")]
    pub async fn reviews(&self, ctx: &Context<'_>,  id: ID) -> Vec<ReviewType> { 
        let id = id.parse::<i32>().expect("");
        resolver::get_reviews_by_product(id, &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|x| ReviewType::from(x))
            .collect()
    }
}

#[derive(Default)]
pub struct QueryReviews;

#[Object]
impl QueryReviews  {
    ///  Insert reference resolvers 
    ///  entities
    #[graphql(entity)]
    pub async fn find_product_id(&self, #[graphql(key)] id: ID) -> ProductType { 
        ProductType { id }
    } 
    /// Resolver Reference 
    #[graphql(entity)]
    pub async fn find_user_by_id(&self, #[graphql(key)] id: ID) -> UserType { 
        UserType { id }
    }
    #[graphql(name = "getReviews")]
    async fn get_all_reviews(&self, ctx: &Context<'_>) -> Vec<ReviewType> { 
        resolver::get_reviews(&get_conn_from_ctx(ctx))
            .expect("s")
            .iter()
            .map(ReviewType::from)
            .collect()
    }
}