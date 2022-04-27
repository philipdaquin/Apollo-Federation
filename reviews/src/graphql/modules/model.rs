use crate::schema::review;
use super::schema::ReviewType;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[table_name = "review"]
pub struct Review { 
    pub id: i32, 
    pub body: String,
    pub author_id: i32, 
    pub product_id: i32
}

impl From<&ReviewType> for Review { 
    fn from(g: &ReviewType) -> Self {
        Review { 
            id:
        }
    }
}