use diesel::prelude::*;
use super::model::Product;
use crate::schema::products;

pub fn get_product_by_id(id: i32, conn: &PgConnection) -> QueryResult<Product> { 
    products::table.filter(products::id.eq(id)).first(conn)
}

pub fn get_all_products(conn: &PgConnection) -> QueryResult<Vec<Product>> { 
    products::table.load(conn)
}