use chrono::NaiveDateTime;
use crate::hooks::get_all_products::{GetAllProductsGetAllProducts, self};
use crate::hooks::get_product_by_id::GetProductByIdGetProductById;
use crate::{
    hooks::{get_product_by_id, GetProductById, use_query, GetAllProducts},
};
use yew::prelude::*;


#[derive(Clone, Debug, PartialEq)]
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


impl From<&GetAllProductsGetAllProducts> for Product { 
    fn from(f: &GetAllProductsGetAllProducts) -> Self {

        let convert = |n: Option<i64> | -> Option<i32> { 
            let n =  n.unwrap() as i32;

            Some(n)
        };
        Self { 
            id: f.id.parse::<i32>().expect(""),
            name: f.name.clone(), 
            price: convert(f.price),
            weight: convert(f.weight),
            category: f.category.clone(),
            created_by: convert(f.created_by),
            tags: f.tags.clone(),
            created_at: f.created_at.clone(),
            updated_at: f.updated_at.clone(),
            description: f.description.clone(),
            image_url: f.image_url.clone()
        }
    }
}



#[derive(Clone, Debug, PartialEq)]
pub struct ProductID { 
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

impl From<&GetProductByIdGetProductById> for ProductID { 
    fn from(f: &GetProductByIdGetProductById) -> Self {

        let convert = |n: Option<i64> | -> Option<i32> { 
            let n =  n.unwrap() as i32;

            Some(n)
        };
        Self { 
            id: f.id.parse::<i32>().expect(""),
            name: f.name.clone(), 
            price: convert(f.price),
            weight: convert(f.weight),
            category: f.category.clone(),
            created_by: convert(f.created_by),
            tags: f.tags.clone(),
            created_at: f.created_at.clone(),
            updated_at: f.updated_at.clone(),
            description: f.description.clone(),
            image_url: f.image_url.clone()
        }
    }
}


impl From<&ProductID> for Product { 
    fn from(k: &ProductID) -> Self {
        Self { 
            id :k.id.clone(), 
            name :k.name.clone(), 
            price :k.price.clone(), 
            weight :k.weight.clone(), 
            category:k.category.clone(),
            created_by:k.created_by.clone(),
            tags :k.tags.clone(), 
            created_at :k.created_at.clone(), 
            updated_at:k.updated_at.clone(),
            description:k.description.clone(), 
            image_url:k.image_url.clone()    
        }
    }
}


