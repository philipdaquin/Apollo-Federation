use yew::{prelude::*, function_component, html, Html};
use crate::components::productlist::Productlist;
#[function_component(FeaturePage)]
pub fn feature_page() -> Html {
    html! {
        <>

            <Productlist/>

            <section class="featured section" id="shop">
                <h2 class="section-title">{"All Products"}</h2>
                <div class="featured__container bd-grid">
                    <article class="sneaker">
                        <img src="assets/img/featured1.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Jordan"}</span>
                        <span class="sneaker__preci">{"$149.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>

                    <article class="sneaker">
                        <img src="assets/img/featured2.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Free RN"}</span>
                        <span class="sneaker__preci">{"$149.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>

                    <article class="sneaker">
                        <img src="assets/img/featured3.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Free RN"}</span>
                        <span class="sneaker__preci">{"$149.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/new2.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Sply"}</span>
                        <span class="sneaker__preci">{"$79.99"}</span>
                        <a href="" class="button-light">{"Add to Cart"} <i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/new3.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Sply"}</span>
                        <span class="sneaker__preci">{"$79.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/new4.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Sply"}</span>
                        <span class="sneaker__preci">{"$79.99"}</span>
                        <a href="" class="button-light">{"Add to Cart"} <i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/women1.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Womens"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/women2.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Womens"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart"} <i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/women3.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Womens"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                    <article class="sneaker">
                        <img src="assets/img/women4.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Womens"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                </div>
                <div class="sneaker__page bd-grid">
                    <div>
                        <span class="sneaker__pag">{"1"}</span>
                        <span class="sneaker__pag">{"2"}</span>
                        <span class="sneaker__pag">{"3"}</span>
                        <span class="sneaker__pag">{"4"}</span>
                        <span class="sneaker__pag">{"&#8594;"}</span>
                    </div>
                </div>
            </section>
        </>
    }
}