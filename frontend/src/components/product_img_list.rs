use yew::{prelude::*, function_component, html, Html};

#[function_component(ProductImgList)]
pub fn product_img_list() -> Html {
    html! {
        <>
            <div class="col-5 cold-md-12">
                <div class="product-img" id="product-img">
                    <img src={image_url.clone()} alt=""/>
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
        </>
    }
}