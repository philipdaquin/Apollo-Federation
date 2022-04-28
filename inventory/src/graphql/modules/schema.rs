use async_graphql::*;
use async_graphql_actix_web::*;
use super::resolver::{get_all_inventory, get_by_product_id, self};
use crate::{graphql::config::get_conn_from_ctx, schema::inventory};

#[derive(SimpleObject)]
pub struct InventoryType { 
    pub id: ID,
    pub weight: Option<i32>, 
    pub price: Option<i32>, 
    pub in_stock: bool,
    pub shipping_estimate: Option<i32> 
}

pub struct ProductType { 
    pub id: ID,
}

#[Object(extends)]
impl ProductType { 
    #[graphql(external)]
    pub async fn id(&self) -> &ID {
        &self.id
    }
    async fn shipping_estimate(&self, ctx: &Context<'_>) -> Option<i32> { 
        let inventory = get_inventory_by_id_internally(self.id.clone(), ctx);
        if inventory.is_some() { 
            return inventory.unwrap().price
        } else { 
            None
        }
    }
    async fn get_by_product_id(&self, ctx: &Context<'_>) -> Option<InventoryType> { 
        get_inventory_by_id_internally(self.id.clone(), ctx)
    }
}
fn get_inventory_by_id_internally(id: ID, ctx: &Context<'_>) -> Option<InventoryType> { 
    let id = id.parse::<i32>().expect("Unable to get Product Id");
    resolver::get_by_product_id(id, &get_conn_from_ctx(ctx))
        .ok()
        .map(|x| InventoryType::from(&x))
}

#[derive(Default)]
struct QueryInventory;

#[Object]
impl QueryInventory { 
    #[graphql(entity)]
    async fn find_product_id(&self, #[graphql(key)] id: ID ) -> ProductType { 
        ProductType { id }
    }
    async fn get_all_inventory(&self,  ctx: &Context<'_>) -> Vec<InventoryType> { 
        resolver::get_all_inventory(&get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| InventoryType::from(f))
            .collect()
    }
}