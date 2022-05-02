use yew::{prelude::*, function_component, html, Html};
use crate::models::products::Product;
use crate::hooks::{use_query::use_query, *, 
    GetAllProducts, get_all_products,  product_query
    
     
};


#[function_component(Productlist)]
pub fn product_list() -> Html {
    let variables = get_all_products::Variables {};
    let get_all_users  = use_query::<GetAllProducts>(variables);
    
    if get_all_users.data.is_none() { 
        return html! {
            <>
                <h1>{"Query Failed!"}</h1>

            </>
        }
    }

    let queried_results: Vec<Product> = get_all_users  
        .data
        .unwrap()
        .get_all_products
        .iter()
        .map(|user| Product::from(user))
        .collect();
    
    let user_info = queried_results
        .iter()
        .map(|info| { 
            return html! { 
                <>
                    <div>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        <h1>{info.id.clone()}</h1>
                        
                    </div>
                </>
            }
        }).collect::<Vec<Html>>();
    

    html! {
        <>
            {user_info}
        </>
    }
}