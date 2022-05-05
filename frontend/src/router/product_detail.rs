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
    } = ProductID::from(&queries_result);

    let on_click = { 
        // add class product-img > img 
    };


    html! {
        <>
            <section class="bd-grid">
                <div class="container">
                    // <div class="box">
                    //     <div class="breadcumb">
                    //         <a href="">{"Home"}</a>
                    //         <span><i class="bx bxs-chevrons-right"></i></span>
                    //         <a href="">{"All Products"}</a>
                    //         <span><i class="bx bxs-chevrons-right"></i></span>
                    //         <a href="">{"Nike Shoes"}</a>
                    //         <span><i class="bx bxs-chevrons-right"></i></span>
                    //     </div>
                    // </div>
                    <div class="row product-row">
                        
                        <div class="col-7 col-md-12">
                            
                                <div class="box">
                                    <div class="box-header">
                                        {"Review"}
                                    </div>
                                    <ReviewList product_id={id} />
                                    <div>
                                        <div class="box">
                                            <ul class="pagination">
                                                <li><a href="#"><i class="bx bxs-chevron-left"></i></a></li>
                                                <li><a href="#" class="active">{"1"}</a></li>
                                                <li><a href="#">{"2"}</a></li>
                                                <li><a href="#">{"3"}</a></li>
                                                <li><a href="#">{"4"}</a></li>
                                                <li><a href="#">{"5"}</a></li>
                                                <li><a href="#"><i class="bx bxs-chevron-right"></i></a></li>
                                            </ul>
                                        </div>
                                    </div>
                                </div>
                                <div class="box">
                                    <div class="box-header">
                                        {"Related Products"}
                                    </div>
                                    <div class="row"></div>
                                </div>
                            </div>
                       
                    </div>                
                </div>
            </section>
        </>
    }
}

