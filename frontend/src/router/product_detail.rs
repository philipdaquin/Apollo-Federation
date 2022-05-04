use yew::{prelude::*, function_component, html, Html};
use crate::hooks::{
    GetProductById, get_product_by_id, use_query
};
use crate::models::products::ProductID;
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
    } = ProductID::from(&queries_result);

    

    html! {
        <>
            <section class="product__details">
                <div class="single_pro_image">
                    <img src="https://media.gq.com/photos/5a7d0307463e0e0b7680173c/master/w_960%2Cc_limit/180207_NYFWMFW18-18_DR13-9741-2.jpg" alt="" width="100%" />

                    <div class="small_img_group">
                        <div class="small_img_col">
                            <img src="https://media.gq.com/photos/5a7d0307463e0e0b7680173c/master/w_960%2Cc_limit/180207_NYFWMFW18-18_DR13-9741-2.jpg" alt="" width= "" class ="small_img"/>
                        </div>
                        <div class="small_img_col">
                            <img src="https://media.gq.com/photos/5a7d0307463e0e0b7680173c/master/w_960%2Cc_limit/180207_NYFWMFW18-18_DR13-9741-2.jpg" alt="" width= "" class ="small_img"/>
                        </div>
                        <div class="small_img_col">
                            <img src="https://media.gq.com/photos/5a7d0307463e0e0b7680173c/master/w_960%2Cc_limit/180207_NYFWMFW18-18_DR13-9741-2.jpg" alt="" width= "" class ="small_img"/>
                        </div>
                        <div class="small_img_col">
                            <img src="https://media.gq.com/photos/5a7d0307463e0e0b7680173c/master/w_960%2Cc_limit/180207_NYFWMFW18-18_DR13-9741-2.jpg" alt="" width= "" class ="small_img"/>
                        </div>
                        <div class="small_img_col">
                            <img src="https://media.gq.com/photos/5a7d0307463e0e0b7680173c/master/w_960%2Cc_limit/180207_NYFWMFW18-18_DR13-9741-2.jpg" alt="" width= "" class ="small_img"/>
                        </div>
                    </div>
                </div>
                <div class="single_pro_details">
                    <h6>{name.clone()}</h6>
                    <h4>{description.clone().unwrap()}</h4>
                    <h2>{format!("${}", price.clone().unwrap())}</h2>
                </div>
            </section>
        </>
    }
}
