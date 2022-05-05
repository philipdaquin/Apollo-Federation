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

    let on_click = { 
        // add class product-img > img 
    };


    html! {
        <>
            <section class="bd-grid">
                <div class="container">
                    <div class="box">
                        <div class="breadcumb">
                            <a href="">{"Home"}</a>
                            <span><i class="bx bxs-chevrons-right"></i></span>
                            <a href="">{"All Products"}</a>
                            <span><i class="bx bxs-chevrons-right"></i></span>
                            <a href="">{"Nike Shoes"}</a>
                            <span><i class="bx bxs-chevrons-right"></i></span>
                        </div>
                    </div>
                    <div class="row product-row">
                        <div class="col-5 cold-md-12">
                            <div class="product-img" id="product-img">
                                <img src="https://static.nike.com/a/images/t_PDP_1280_v1/f_auto,q_auto:eco/fb7eda3c-5ac8-4d05-a18f-1c2c5e82e36e/blazer-mid-77-vintage-shoe-CBDjT0.png" alt=""/>
                            </div>
                            <div class="box">
                                <div class="product-img-list">
                                    <div class="product-img-item">
                                        <img src="https://static.nike.com/a/images/t_PDP_1280_v1/f_auto,q_auto:eco/e1f4280c-7c89-4c5f-a4c9-67e3ab698051/blazer-mid-77-vintage-womens-shoes-25h71W.png" alt=""/>
                                    </div>
                                    <div class="product-img-item">
                                        <img src="https://static.nike.com/a/images/t_PDP_1280_v1/f_auto,q_auto:eco/fb7eda3c-5ac8-4d05-a18f-1c2c5e82e36e/blazer-mid-77-vintage-mens-shoes-nw30B2.png" alt=""/>
                                    </div>
                                    <div class="product-img-item">
                                        <img src="https://static.nike.com/a/images/t_PDP_1280_v1/f_auto,q_auto:eco/e1f4280c-7c89-4c5f-a4c9-67e3ab698051/blazer-mid-77-vintage-womens-shoes-25h71W.png" alt=""/>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="col-7 col-md-12">
                            <div class="product-info">
                                <h1>
                                    {"NIKE SHOES"}
                                </h1>
                                <div class="product-info-detail">
                                    <span class="product-info-detail-title">{"BRAND"}</span>
                                    <a href="">{"NIKE"}</a>
                                </div>
                                <div class="product-info-detail">
                                    <span class="product-info-detail-title">{"RATED"}</span>
                                    <span class="rating">
                                        <i class="bx bxs-star"></i>
                                        <i class="bx bxs-star"></i>
                                        <i class="bx bxs-star"></i>
                                        <i class="bx bxs-star"></i>
                                        <i class="bx bxs-star"></i>
                                    </span>
                                </div>
                                <p class="product-description">
                                {"Lorem ipsum dolor sit amet consectetur adipisicing elit. 
                                Veritatis cupiditate, sint alias officiis ipsum modi at nesciunt quos 
                                dolorum dolores vero nemo quaerat necessitatibus dicta culpa tenetur magnam, 
                                autem dignissimos."}</p>
                                <div class="product-info-price">{"$2345"}</div>
                                <div class="product-quantity-wrapper">
                                    <span class="product-quantity-btn">
                                        <i class="bx bx-minus"></i>
                                    </span>
                                    <span class="product-quantity">{"1"}</span>
                                    <span class="product-quantity-btn">
                                        <i class="bx bx-plus"></i>
                                    </span>
                                </div>
                                <div>
                                    <button class="btn-flat btn-hover">{"Add to Card"}</button>
                                </div>
                                <div class="box">
                                    <div class="box-header">
                                        {"Description"}
                                    </div>
                                    <div class="product-detail-description">
                                        <button class="btn-flat btn-hover btn-view-description">
                                            {"View All"}
                                        </button>
                                        <div class="product-detaul-description-content">
                                            <p>
                                                {"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aut, vero."}
                                            </p>
                                            <img src="https://static.nike.com/a/images/t_PDP_1280_v1/f_auto,q_auto:eco/e1f4280c-7c89-4c5f-a4c9-67e3ab698051/blazer-mid-77-vintage-womens-shoes-25h71W.png" alt=""/>
                                            <p>
                                                {"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aut, vero."}
                                            </p>
                                            <img src="https://static.nike.com/a/images/t_PDP_1280_v1/f_auto,q_auto:eco/e1f4280c-7c89-4c5f-a4c9-67e3ab698051/blazer-mid-77-vintage-womens-shoes-25h71W.png" alt=""/>
                                            <p>
                                                {"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Aut, vero."}
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>                
                </div>
            </section>
        </>
    }
}
