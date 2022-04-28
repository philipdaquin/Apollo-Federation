use crate::schema::inventory;
use super::schema::InventoryType;

#[derive(Debug, Clone, PartialEq, Identifiable, Queryable)]
#[table_name = "inventory"]
pub struct Inventory { 
    pub id: i32, 
    pub weight: Option<i32>,
    pub price: Option<i32>, 
    pub in_stock: bool,
    pub shipping_estimate: Option<i32>
}

impl From<&Inventory> for InventoryType { 
    fn from(f: &Inventory) -> Self {
        Self { 
            id: f.id.into(),
            weight: f.weight,
            price: f.price,
            in_stock: f.in_stock,
            shipping_estimate: f.shipping_estimate
        }
    }
}