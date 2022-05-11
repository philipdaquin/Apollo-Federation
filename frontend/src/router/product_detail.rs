use yew::{prelude::*, function_component, html, Html};
use crate::hooks::{
    GetProductById, get_product_by_id, use_query
};
use crate::models::products::ProductID;
use crate::components::review_list::ReviewList;
use yew_router::prelude::*;
use crate::router::AppRoute;
use yew_hooks::use_counter;

#[derive(Properties, Clone, PartialEq)]
pub struct ProductProps { 
    #[prop_or_default]
    pub id: i32
}

#[function_component(ProductDetail)]
pub fn product_detail(ProductProps {id}: &ProductProps ) -> Html {

    let variables = get_product_by_id::Variables { 
        get_product_by_id_id: id.to_string()
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
    let ProductID {
        id, 
        name, 
        price, 
        weight, 
        category,
        created_by,
        tags,
        created_at,
        updated_at,
        description,
        image_url
    } = ProductID::from(&queries_result).clone();

    let updated_time = if updated_at.is_some() { 
        html! {
            <p>{
                updated_at.unwrap().format("%B %e, %Y")
            }</p>
        } 
    } else { 
        html! {} 
    };


    let quantity = use_counter( 1);
    let add_quantity = {
        let quantity = quantity.clone();
        Callback::from(move |_| quantity.increase())
    };
    let decre_quantity = { 
        let quantity = quantity.clone();
        Callback::from(move |_| quantity.decrease())
    };

    html! {
        <>
            <section class="featured section bd-grid">
                <div class="bg-main ">
                    <div class="box">
                        <div class="breadcumb">
                            <Link<AppRoute> to={AppRoute::Home}>{"Home"}</Link<AppRoute>> 
                            <span><i class="bx bxs-chevrons-right"></i></span>
                            <Link<AppRoute> to={AppRoute::FeaturePage}>{"All Products"}</Link<AppRoute>> 
                            <span><i class="bx bxs-chevrons-right"></i></span>
                            <Link<AppRoute> to={AppRoute::ProductPage { id }}>{name.clone().to_ascii_uppercase()}</Link<AppRoute>> 
                        </div>
                    </div>
                    <div class="row product-row">
                        <div class="col-5 col-md-12">
                            <div class="product-img">
                                <img src="https://www.thewrap.com/wp-content/uploads/2017/12/Screen-Shot-2017-12-04-at-8.52.42-AM-618x400.png" alt=""/>
                            </div>
                            <div class="box">
                                <div class="product-img-list">
                                    <div class="product-img-item">
                                        <img src="https://www.thewrap.com/wp-content/uploads/2017/12/Screen-Shot-2017-12-04-at-8.52.42-AM-618x400.png" alt=""/>
                                    </div> 
                                    <div class="product-img-item">
                                        <img src="https://www.thewrap.com/wp-content/uploads/2017/12/Screen-Shot-2017-12-04-at-8.52.42-AM-618x400.png" alt=""/>
                                    </div> 
                                    <div class="product-img-item">
                                        <img src="https://www.thewrap.com/wp-content/uploads/2017/12/Screen-Shot-2017-12-04-at-8.52.42-AM-618x400.png" alt=""/>
                                    </div> 
                                </div>
                            </div>
                        </div>
                        <div class="col-7 col-md-12">
                            <div class="product-info">
                                <h1>{name}</h1>
                            </div>
                            <div class="product-info-detail">
                                <span class="product-info-detail-title">{format!("Brand:")}</span>
                                <a href="">{"NIKE"}</a>
                            </div>
                            <div class="product-info-detail">
                                <span class="product-info-detail-title">{"Rated: "}</span>
                                <span class="rating">
                                    <i class="bx bxs-star"></i>
                                    <i class="bx bxs-star"></i>
                                    <i class="bx bxs-star"></i>
                                    <i class="bx bxs-star"></i>
                                    <i class="bx bxs-star"></i>
                                </span>
                            </div>
                            <div class="product-description">
                                <p>{format!("{}", description.expect("Unable to get product description"))}</p>
                                <div class="product-info-price">{format!("${}", price.expect("Unable to get the price of product"))}</div>
                                <div class="product-quantity-wrapper">
                                    <button class="product-quantity-btn" onclick={decre_quantity}>
                                        <i class="bx bx-minus"></i>
                                    </button>
                                    <span class="product-quantity">{*quantity}</span>
                                    <button class="product-quantity-btn" onclick={add_quantity}>
                                        <i class="bx bx-plus"></i>
                                    </button>
                                </div>
                                <button class="btn-flat btn-hover">{"Add to Cart"}</button>
                            </div>
                        </div>
                    </div>
                    //  Insert Product Reviews 
                    <ReviewList product_id={id}/>
                    // Related Products 
                </div>
            </section>
        </>
    }
}
