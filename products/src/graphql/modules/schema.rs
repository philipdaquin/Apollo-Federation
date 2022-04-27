
use {
    super::resolver::{get_all_products, get_product_by_id, self},
    async_graphql::*,
    async_graphql_actix_web::*,
    super::model::Product,
    crate::{graphql::config::get_conn_from_ctx}
};

#[derive(Default)]
pub struct QueryProducts;

///  The Price Type in our System 
#[derive(SimpleObject)]
pub struct ProductType { 
    pub id: ID,
    pub name: String, 
    /// Price attribute can be Zero 
    pub price: Option<i32>, 
    pub weight: Option<i32>
}


#[Object]
impl QueryProducts { 
    /// Get all products found inside the Database
    #[graphql(name = "getAllProducts")]
    pub async fn get_all(&self, ctx: &Context<'_>) -> Vec<ProductType> { 
        resolver::get_all_products(&get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| ProductType::from(f))
            .collect()
    }
    /// Reference Resolver 
    #[graphql(entity)]
    pub async fn find_product_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<ProductType> { 
        let id = id.parse::<i32>().expect("");
        resolver::get_product_by_id(id, &get_conn_from_ctx(ctx))
            .ok()
            .map(|f| ProductType::from(&f))
    }

}

impl From<&Product> for ProductType { 
    fn from(f: &Product) -> Self {
        ProductType { 
            id: f.id.into(), 
            name: f.name.clone(), 
            price: f.price.clone(), 
            weight: f.weight.clone()
        }
    }
}