use yew::{prelude::*, function_component, html, Html};


/// Show Product Information in Details 
#[function_component(ProductInfo)]
pub fn product_info() -> Html {
    html! {
        <>
            <section>
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
                    {
                        "Lorem ipsum dolor sit amet consectetur adipisicing elit. 
                Veritatis cupiditate, sint alias officiis ipsum modi at nesciunt quos 
                dolorum dolores vero nemo quaerat necessitatibus dicta culpa tenetur magnam, 
                autem dignissimos."
                
                    }
                </p>
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
            </section>
        </>
    }
}

                           