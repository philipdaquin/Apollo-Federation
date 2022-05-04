use async_graphql::*;
use async_graphql_actix_web::*;
use chrono::NaiveDateTime;
use super::resolver::{get_reviews, get_reviews_by_product, get_reviews_by_user, self};
use crate::graphql::config::get_conn_from_ctx;
use super::model::{Review, NewReview};

#[derive(SimpleObject)]
pub struct ReviewType { 
    pub id: ID,
    pub body: String,
    pub author: UserType,
    pub product: ProductType,
    pub heading: String, 
    pub media: Option<String>,
    pub is_edited: bool
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



#[derive(Default)]
pub struct MutationReviews;

#[derive(InputObject, Clone, Debug)]
pub struct NewReviewInput { 
    pub body: String, 
    pub author_id: ID, 
    pub product_id: ID,
    pub heading: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub media: Option<String>,
    pub is_edited: Option<bool>
}

#[Object]
impl MutationReviews { 
    pub async fn create_new_review(&self, ctx: &Context<'_>, new_review: NewReviewInput) -> FieldResult<ReviewType> { 
        let review = resolver::create_review(
            NewReview::from(&new_review), 
                &get_conn_from_ctx(ctx)).expect("");
        Ok(ReviewType::from(&review))
    }
    pub async fn update_review(
        &self, 
        ctx: &Context<'_>, 
        new_review: NewReviewInput,
        review_id: ID, 
        author_id: ID
    ) -> FieldResult<ReviewType> { 
        let parse = |x: ID| -> i32 { 
            x.parse::<i32>().expect("ParseIntError")
        };
        let review = resolver::update_review(
            NewReview::from(&new_review), 
            parse(review_id), 
            parse(author_id), 
            &get_conn_from_ctx(ctx)
        ).expect("");
    
        Ok(ReviewType::from(&review))

    }
    pub async fn delete_review(&self, ctx: &Context<'_>, review_id: ID) -> FieldResult<bool> { 
        let id = review_id.parse::<i32>().expect("ParseIntError");
        resolver::delete_review(id, &get_conn_from_ctx(ctx));

        Ok(true)
    }

}