use chrono::NaiveDateTime;
use crate::hooks::get_reviews_of_any_product_id::GetReviewsOfAnyProductIdGetReviewsOfAnyProductId;
#[derive(Clone, Debug, PartialEq)]
pub struct Review { 
    pub id: i32, 
    pub body: String,
    pub author_id: AuthorData, 
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
    pub username: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReviewResponseData { 
    pub id: i32, 
    pub body: String,
    pub heading: Option<String>, 
    pub media: Option<String>,
    pub is_edited: Option<bool>,
    pub user_rating: Option<i64>,
    pub author: AuthorData
}

impl From<&GetReviewsOfAnyProductIdGetReviewsOfAnyProductId> for ReviewResponseData { 
    fn from(f: &GetReviewsOfAnyProductIdGetReviewsOfAnyProductId) -> Self {

        let convert = |n: Option<i64> | -> Option<i32> { 
            let n =  n.unwrap() as i32;

            Some(n)
        };

        let author = AuthorData { 
            id: f.author.id.parse::<i32>().expect(""),
            username: f.author.username,
            first_name: f.author.first_name, 
            last_name: f.author.last_name
        };


        Self { 
            id: f.id.parse::<i32>().expect(""),
            body: f.body,
            heading: Some(f.heading),
            media: f.media,
            is_edited: Some(f.is_edited),
            user_rating: f.user_rating,
            author
        }
    }
}


