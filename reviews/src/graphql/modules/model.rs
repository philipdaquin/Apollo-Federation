use crate::schema::review;
use super::schema::{ReviewType, UserType, ProductType};
use async_graphql::*;
use chrono::NaiveDateTime;
use super::schema::NewReviewInput;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[table_name = "review"]
pub struct Review { 
    pub id: i32, 
    pub body: String,
    pub author_id: i32, 
    pub product_id: i32,
    pub heading: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub media: Option<String>,
    pub is_edited: Option<bool>,
    pub created_at: Option<NaiveDateTime>
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[table_name = "review"]
pub struct NewReview { 
    pub body: String, 
    pub author_id: i32, 
    pub product_id: i32,
    pub heading: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub media: Option<String>,
    pub is_edited: Option<bool>
}

/// Read Database type into Graphql type 
impl From<&Review> for ReviewType { 
    fn from(g: &Review) -> Self {
        ReviewType { 
            id: g.id.into(),
            body: g.body.clone(),
            author: UserType::new(g.author_id.clone()),
            product: ProductType::new(g.product_id.clone()),
            heading: g.heading.clone().unwrap(),
            media: g.media.clone(),
            is_edited: g.is_edited.unwrap_or(false)
        }
    }
}
/// Convert Graphql type into Review type 
impl From<&NewReviewInput> for NewReview { 
    fn from(f: &NewReviewInput) -> Self {
        Self { 
            body: f.body.clone(),
            author_id: f.author_id.parse::<i32>().expect(""),
            product_id: f.product_id.parse::<i32>().expect(""),
            heading: f.heading.clone(), 
            updated_at: f.updated_at.clone(),
            media: f.media.clone(),
            is_edited: f.is_edited.clone()
        }
    }
}


impl UserType { 
    fn new(id: i32) -> Self { 
        Self { 
            id: id.into()
        }
    }
}
impl ProductType { 
    fn new(id: i32) -> Self { 
        Self { 
            id: id.into()
        }
    }
}


