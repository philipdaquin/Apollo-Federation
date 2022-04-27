use crate::schema::products;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[table_name = "products"]
pub struct Product { 
    pub id: i32, 
    pub name: String,
    pub price: Option<i32>, 
    pub weight: Option<i32>,
}
