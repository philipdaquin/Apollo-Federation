use yew::{prelude::*, function_component, html, Html};
use crate::hooks::{
    GetProductById, get_product_by_id, use_query
};
use crate::models::products::ProductID;
use crate::components::review_list::ReviewList;



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

    html! {
        <>
            <section class="bd-grid">
                <div class="small-container single-product">
                    <div class="row">
                        <div class="col-2">
                            <img
                                width="100%" 
                                src="https://i.ytimg.com/vi/kgUDbvKYbWk/maxresdefault.jpg" 
                                alt=""
                                id="product-img"
                            />
                            <div class="small-img-row">
                                <div class="small-img-col">
                                    <img 
                                        width="100%"
                                        src="https://www.thewrap.com/wp-content/uploads/2017/12/Screen-Shot-2017-12-04-at-8.52.42-AM.png" 
                                        alt=""
                                        class="small-img"
                                    />
                                </div>
                                <div class="small-img-col">
                                    <img 
                                        width="100%"
                                        src="https://www.thewrap.com/wp-content/uploads/2017/12/Screen-Shot-2017-12-04-at-8.52.42-AM.png" 
                                        alt=""
                                        class="small-img"
                                    />
                                </div>
                                <div class="small-img-col">
                                    <img 
                                        width="100%"
                                        src="https://www.thewrap.com/wp-content/uploads/2017/12/Screen-Shot-2017-12-04-at-8.52.42-AM.png" 
                                        alt=""
                                        class="small-img"
                                    />
                                </div>
                            </div>
                        </div>
                        <div class="col-2">
                            <p>{format!("/product/{}-{}", name.clone(), id.clone())}</p>
                            <h1>{name}</h1>
                            <h4>{format!("${} USD", price.unwrap_or(0))}</h4>
                            <select>
                                <option>{"Select Size"}</option>
                                <option>{"XXL"}</option>
                                <option>{"XL"}</option>
                                <option>{"Large"}</option>
                                <option>{"Medium"}</option>
                                <option>{"Small"}</option>
                            </select>
                            <input type="number" value="1" />
                            <button class="button">{"Add to Cart"}</button>

                            <h3>{"Product Details"}</h3>
                            <br/>
                            <p>{format!("{}", description.expect("Unable to get product description"))}</p>
                        </div>
                    </div>
                 
                    <div class="box">
                        <h3>{weight.unwrap_or(0)}</h3>
                        <h3>{created_by.unwrap_or(0)}</h3>
                        <p>{tags.unwrap()}</p>
                        <h4>{category.unwrap()}</h4>
                        <p>{created_at.unwrap().format("%B %e, %Y")}</p>
                        <p>{updated_time}</p>
                        <img src={image_url.unwrap()} alt=""/>
                    </div>

                    //  Insert Product Reviews 
                    <ReviewList product_id={id}/>

                    // Related Products 
                               
                </div>
            </section>
        </>
    }
}

fn render_check<T>(item: Option<T>) -> bool { 
    match item { 
        Some(_) => return false,
        _ => return true
    }
}