use yew::{prelude::*, function_component, html, Html};
use crate::hooks::{user_query};
use crate::models::user::UserInfo;
use crate::components::{
    header::HeaderObject,
    collection::ColletionObject,
    womens::WomensObject,
    newsletter::NewsletterObject,
    product_list::Productlist,
    product_review::ProductReview

};

#[function_component(Home)]
pub fn home_page() -> Html {
    html! {
        <>
            <section>
                <HeaderObject/>
                <section class="featured section" id="featured">
                    <h2 class="section-title">{"FEATURED"}</h2>
                    <div class="featured__container bd-grid">
                        <ProductReview product_id={4} show_tags={true}/>
                        <ProductReview product_id={5} show_tags={true}/>
                        <ProductReview product_id={6} show_tags={true}/>
                    </div>
                </section>
                <ColletionObject/>
                <WomensObject/>
                <section class="offer section">
                    <div class="offer__container bd-grid">
                        <div class="offer__data">
                            <h3 class="offer__title">{"50%"}</h3>
                            <p class="offer__description">{"in Adidas Superstar sneakers"}</p>
                            <a href="#" class="button">{"Shop Now"}</a>
                        </div>
                        <img src="../../assets/img/offert.png" alt="" class="offer__img"/>            
                    </div>
                </section>
            </section>



            <section class="new section" id="new">
                <h2 class="section-title">{"NEW COLLECTION"}</h2>
                <div class="new__container bd-grid">
                    <div class="new__mens">
                        <img src="../../assets/img/new1.png" alt="" class="new__mens-img"/>
                        <h3 class="new__title">{"Mens Shoes"}</h3>
                        <span class="new__preci">{"From 179.99"}</span>
                        <a href="#" class="button-light">{"View Collection "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </div>
                    <div class="new__sneaker">
                        <div class="new__sneaker-card">
                            <img src="../../assets/img/new2.png" alt="" class="new__sneaker-img"/>
                            <div class="new__sneaker-overlay">
                                <a href="#" class="button">{"Add to Cart"}</a>
                            </div>
                        </div>

                        <div class="new__sneaker-card">
                            <img src="../../assets/img/new3.png" alt="" class="new__sneaker-img"/>
                            <div class="new__sneaker-overlay">
                                <a href="#" class="button">{"Add to Cart"}</a>
                            </div>
                        </div>
                        <div class="new__sneaker-card">
                            <img src="../../assets/img/new4.png" alt="" class="new__sneaker-img"/>
                            <div class="new__sneaker-overlay">
                                <a href="#" class="button">{"Add to Cart"}</a>
                            </div>
                        </div>
                        
                        <div class="new__sneaker-card">
                            <img src="../../assets/img/new5.png" alt="" class="new__sneaker-img"/>
                            <div class="new__sneaker-overlay">
                                <a href="#" class="button">{"Add to Cart"}</a>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <NewsletterObject/>
        </>
    }
}