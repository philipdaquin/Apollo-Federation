use crate::schema::review;
use super::schema::{ReviewType, UserType, ProductType};
use async_graphql::*;
use chrono::NaiveDateTime;

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
    pub is_edited: Option<bool>
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



impl From<&Review> for ReviewType { 
    fn from(g: &Review) -> Self {
        ReviewType { 
            id: g.id.into(),
            body: g.body.clone(),
            author: UserType::new(g.author_id.clone()),
            product: ProductType::new(g.product_id.clone()),
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