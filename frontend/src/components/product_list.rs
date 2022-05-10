use yew::{prelude::*, function_component, html, Html};
use crate::models::products::Product;
use crate::hooks::{use_query::use_query,  
    GetAllProducts, get_all_products,  user_query
};

use crate::router::AppRoute;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ProductListProps { 
    pub filter: ProductListFilter 
}

#[derive(Clone, PartialEq, Debug)]
pub enum ProductListFilter { 
    All,
    ByAuthorId(i32),
    ByTag(String),
    ByCategory(String)
}


#[function_component(Productlist)]
pub fn product_list(ProductListProps {filter}: &ProductListProps) -> Html {
    let variables = get_all_products::Variables {};
    let get_all_products  = use_query::<GetAllProducts>(variables);
    
    if get_all_products.data.is_none() { 
        return html! {
            <>
                <h1>{"Query Failed!"}</h1>
            </>
        }
    }
    let mut queried_results: Vec<Product> = get_all_products
        .data
        .unwrap()
        .get_all_products
        .iter()
        .map(|user| Product::from(user))
        .collect();

    // let filtered: Vec<Product> = match filter { 
        
    //     ProductListFilter::ByAuthorId(author_id) => { 
    //         queried_results.clone().retain(|product| *author_id == product.created_by.unwrap());
    //     }, 
    //     ProductListFilter::ByTag(tag) => { 
    //         queried_results.retain(|product| *tag == *product.tags.clone().unwrap().to_string());
    //     },
    //     ProductListFilter::ByCategory(category) => { 
    //         queried_results.retain(|product| *category == *product.tags.clone().unwrap());
    //     },
    //     ProductListFilter::All => { 
    //         queried_results
    //     }
    // };
    

    let user_info = queried_results.clone()
        .iter()
        .map(|info| { 
            return html! { 
                <>
                    <article class = "sneaker">
                        
                        <Link<AppRoute> to={AppRoute::ProductPage {id: info.id}}>
                            <img src={info.image_url.clone()} class="sneaker__img" alt=""/>
                        </Link<AppRoute>>
                        
                        <span class="sneaker__name">{info.name.clone()}</span>
                        <span class="sneaker__preci">{format!("${:?}", info.price.unwrap_or_default())}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                </>
            }
        }).collect::<Vec<Html>>();
    

    html! {
        <>
            {user_info}
        </>
    }
}

