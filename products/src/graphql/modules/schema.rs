use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

use super::model::NewProduct;
use {
    super::resolver::{get_all_products, get_product_by_id, self},
    async_graphql::*,
    async_graphql_actix_web::*,
    super::model::Product,
    crate::{graphql::config::get_conn_from_ctx},
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
    pub weight: Option<i32>,
    pub category: Option<String>, 
    pub created_by: Option<i32>,
    pub tags: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub image_url: Option<String>
}

pub struct UserType { 
    pub id: ID
}

#[Object(extends)]
impl UserType { 
    #[graphql(external)]
    pub async fn id(&self) -> &ID { &self.id}

    #[graphql(name = "getUserProducts")]
    pub async fn products(&self, ctx: &Context<'_>, id: ID) -> Vec<ProductType> { 
        let id = id.parse::<i32>().expect("");
        resolver::get_all_products_by_id(id, &get_conn_from_ctx(ctx))
            .expect("")
            .iter()
            .map(|f| ProductType::from(f))
            .collect()
    }
}

#[Object(extends)]
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
        find_product_by_id_internal(ctx, id)
    }
    #[graphql(name = "getShippingEstimate")]
    pub async fn shipping_estimate(&self, ctx: &Context<'_>, id: ID) -> Option<i32> { 
        let ProductType { 
            price, 
            weight, .. 
        }  = find_product_by_id_internal(ctx, id).unwrap(); 
        Some(price.unwrap_or_default() * weight.unwrap_or_default())
    } 
    #[graphql(entity)]
    pub async fn find_user_by_id(&self, #[graphql(key)] id: ID) -> UserType { 
        UserType { id }
    }

}
fn find_product_by_id_internal(ctx: &Context<'_>, id: ID) -> Option<ProductType> { 
    let id = id.parse::<i32>().expect("");
    resolver::get_product_by_id(id, &get_conn_from_ctx(ctx))
        .ok()
        .map(|f| ProductType::from(&f))
}

#[derive(Default)]
pub struct MutateProduct;

#[derive(InputObject, Clone, Debug)]
pub struct NewProductInput { 
    pub name: String,
    pub price: Option<i32>, 
    pub weight: Option<i32>,
    pub category: Option<String>, 
    pub created_by: Option<i32>,
    pub tags: Option<String>,
    /// users should not be able tto change the time manually 
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub image_url: Option<String>
}


#[Object]
impl MutateProduct { 
    #[graphql(name = "createNewProduct")]
    async fn create_product(&self, ctx: &Context<'_>, new_product: NewProductInput) -> Option<ProductType> { 
        let product = resolver::create_product(
            NewProduct::from(&new_product), 
            &get_conn_from_ctx(ctx))
            .expect("Unable to Execute 'Create_Product' in the Database");
        ProductType::from(&product).into()
            
    }    
    #[graphql(name = "updateProduct")]
    async fn update_product(
        &self, 
        ctx: &Context<'_>, 
        product_id: ID, 
        user_id: ID, 
        new_input: NewProductInput) -> Option<ProductType> { 
        let parse = |x: ID| -> i32 { 
            x.parse::<i32>().expect("Unable to Parse ID")
        };

        let new_product_input = NewProductInput { 
            updated_at: chrono::Utc::now().naive_utc().into(), 
            ..new_input.clone()
        };

        let product = resolver::update_product(
            parse(product_id), 
            parse(user_id), 
            NewProduct::from(&new_product_input), 
            &get_conn_from_ctx(ctx)).expect("");
        ProductType::from(&product).into()
    }
    #[graphql(name = "deleteProduct")]
    async fn delete_product(&self, ctx: &Context<'_>, product_id: ID) -> FieldResult<bool> { 
        Ok(resolver::delete_product(product_id.parse()?, &get_conn_from_ctx(ctx)).expect(""))
       
    }
}







