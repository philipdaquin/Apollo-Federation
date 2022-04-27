use async_graphql::*;
use async_graphql_actix_web::*;
use super::resolver::{get_reviews, get_reviews_by_product, get_reviews_by_user, self};
use crate::graphql::config::get_conn_from_ctx;

#[derive(SimpleObject)]
pub struct ReviewType { 
    pub id: ID,
    pub body: String,
    pub author: User,
    pub product: Product 
}


pub struct User { 
    /// external
    pub id: ID
}
#[Object(extends)]
impl User { 
    #[graphql(external)]
    async fn user_id(&self) -> &ID { 
        &self.id
    }


}


pub struct Product { 
    /// External
    pub id: ID
}
#[Object(extends)]
impl Product { 
    #[graphql(external)]
    async fn product_id(&self) -> &ID { 
        &self.id
    }    
    async fn reviews(&self, ctx: &Context<'_>, id: ID) -> Vec<ReviewType> { 
        let id = id.parse::<i32>().expect("");
        resolver::get_reviews_by_product(id, &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|x| ReviewType::from(&f))
            .collect()
    }
}



#[derive(Default)]
struct QueryReviews;
#[Object]
impl QueryReviews  {
    ///  Insert reference resolvers 
    ///  entities
    #[graphql(entity)]
    async fn find_product_id(&self, id: ID) -> Product { 
        Product { id }
    } 
    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User { 
        User { id }
    }
}