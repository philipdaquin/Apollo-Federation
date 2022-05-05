use crate::hooks::get_product_by_id_with_reviews::GetProductByIdWithReviewsGetProductById;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq)]
pub struct Review { 
    pub id: i32, 
    pub body: String,
    pub author_id: i32, 
    pub product_id: i32,
    pub heading: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub media: Option<String>,
    pub is_edited: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub user_rating: Option<i32>
}

pub struct ReviewResponseData { 

}