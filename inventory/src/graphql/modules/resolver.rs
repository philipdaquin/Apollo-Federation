use crate::schema::inventory;
use diesel::prelude::*;
use super::model::Inventory;


pub fn get_all_inventory(conn: &PgConnection) -> QueryResult<Vec<Inventory>> { 
    inventory::table.load(conn)
} 
pub fn get_by_product_id(id: i32, conn: &PgConnection) -> QueryResult<Inventory> { 
    inventory::table.filter(inventory::id.eq(id)).first(conn)
}