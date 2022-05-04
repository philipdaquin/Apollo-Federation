use yew::{prelude::*, function_component, html, Html};
use crate::hooks::{
    GetProductById, get_product_by_id, use_query
};
use crate::models::products::ProductID;
use crate::router::AppRoute;
use yew_router::prelude::*;


#[derive(Properties, PartialEq, Clone)]
pub struct PreviewProp { 
    #[prop_or_default]
    pub product_id: i32,
    #[prop_or_default]
    pub show_tags: bool
}

#[function_component(ProductReview)]
pub fn product_review(PreviewProp { product_id, show_tags}: &PreviewProp) -> Html {
    
    let variables = get_product_by_id::Variables { 
        get_product_by_id_id: product_id.to_string()
    };
    let get_product = use_query::<GetProductById>(variables);
    if get_product.data.is_none() { 
        return html! { 
            <>
                <h2>{"Unable to fetch product_id"}</h2>
            </>
        }
    }
    let queries_result = get_product   
        .data
        .unwrap()
        .get_product_by_id
        .unwrap();

    let info = ProductID::from(&queries_result);

    html! {
        <>
            <div class="featured__container bd-grid">
                <article class = "sneaker">
                    <div hidden={!
                        *show_tags} class="sneaker__sale">{"Sale"}</div>

                    <Link<AppRoute> to={AppRoute::ProductPage {id: *product_id}}>
                        <img src={info.image_url.clone()} class="sneaker__img" alt=""/>
                    </Link<AppRoute>>
                    
                    <span class="sneaker__name">{info.name.clone()}</span>
                    <span class="sneaker__preci">{format!("${:?}", info.price.unwrap_or_default())}</span>
                    <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                </article>
            </div>
        </>
    }
}