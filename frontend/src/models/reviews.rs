use chrono::NaiveDateTime;
use crate::hooks::get_reviews_of_any_product_id::GetReviewsOfAnyProductIdGetReviewsOfAnyProductId;
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


#[derive(Debug, Clone, PartialEq)]
pub struct AuthorData { 
    pub id: i32,
    pub username: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReviewResponseData { 
    pub id: i32,
    pub body: String,
    // pub author: AuthorData,
    pub heading: String, 
    pub media: Option<String>, 
    pub is_edited: Option<bool>, 
    pub user_rating: Option<i32>
}

impl From<&GetReviewsOfAnyProductIdGetReviewsOfAnyProductId> for ReviewResponseData { 
    fn from(f: &GetReviewsOfAnyProductIdGetReviewsOfAnyProductId) -> Self {

        let convert = |n: Option<i64> | -> Option<i32> { 
            let n =  n.unwrap() as i32;

            Some(n)
        };
        Self { 
            id: f.id.parse::<i32>().expect(""), 
            body: f.body.clone(),
            // author: AuthorData { 
            //     id: f.author.id.parse::<i32>().expect(""),
            //     username: f.author.username.clone(),
            // },
            heading: f.heading.clone(),
            media: f.media.clone(),
            is_edited: Some(f.is_edited),
            user_rating: convert(f.user_rating)
        }
    }
}


