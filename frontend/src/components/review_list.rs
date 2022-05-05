use yew::{prelude::*, function_component, html, Html};


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ReviewProps { 
    pub product_id: i32
}


#[function_component(ReviewList)]
pub fn review_list(ReviewProps {product_id}: &ReviewProps) -> Html {
    html! {
        <>
            <div class="user-rate">
                <div class="user-info">
                    <div class="user-avt">
                        <img src="https://imageio.forbes.com/specials-images/imageserve/61578d13c3a591704133bc40/Header-TrumpL-1x1/0x0.gif?format=gif&height=1080&width=1080&fit=crop" alt=""/>
                    </div>
                    <div class="user-name">
                    <span class="name">{"Donald Trumpet"}</span>
                        <span class="rating">
                            <i class="bx bxs-star"></i>
                            <i class="bx bxs-star"></i>
                            <i class="bx bxs-star"></i>
                            <i class="bx bxs-star"></i>
                            <i class="bx bxs-star"></i>
                        </span>
                    </div>
                </div>
                <div class="user-rate-content">
                    {"Lorem ipsum dolor sit amet consectetur adipisicing elit.
                    Distinctio ea iste, veritatis nobis amet illum, cum alias magni dolores odio, 
                    eius quo excepturi veniam ipsa voluptatibus natus voluptas vero? Aspernatur!"}
                </div>
            </div>
        </>
    }
}