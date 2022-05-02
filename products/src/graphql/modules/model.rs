use crate::schema::products;
use chrono::{NaiveDateTime, NaiveDate};
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[table_name = "products"]
pub struct Product { 
    pub id: i32, 
    pub name: String,
    pub price: Option<i32>, 
    pub weight: Option<i32>,
    pub category: Option<String>, 
    pub created_by: Option<i32>,
    pub tags: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub image_url: Option<String>
}

#[derive(Debug, Clone, AsChangeset, Insertable)]
#[table_name = "products"]
pub struct NewProduct { 
    pub name: String,
    pub price: Option<i32>, 
    pub weight: Option<i32>,
    pub category: Option<String>, 
    pub created_by: Option<i32>,
    pub tags: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub image_url: Option<String>
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShoppingCart { 
    id: i32,
    session_id: i32, 
    product_id: i32, 
    quantity: i32, 
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime 
}

#[derive(Clone, Debug)]
pub struct ShoppingSession { 
    id: i32, 
    user_id: i32, 
    total: i32, 
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}