use yew::{prelude::*, function_component, html, Html};

#[function_component(FeatureObject)]
pub fn feature_component() -> Html {
    html! {
        <>
            <section class="featured section" id="featured">
                <h2 class="section-title">{"FEATURED"}</h2>
                <div class="featured__container bd-grid">
                    <article class="sneaker">
                        <div class="sneaker__sale">{"Sale"}</div>
                        <img src="../../assets/img/featured1.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Jordan"}</span>
                        <span class="sneaker__preci">{"$149.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>

                    <article class="sneaker">
                        <div class="sneaker__sale">{"Sale"}</div>
                        <img src="../../assets/img/featured2.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Free RN"}</span>
                        <span class="sneaker__preci">{"$149.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>

                    <article class="sneaker">
                        <div class="sneaker__sale">{"Sale"}</div>
                        <img src="../../assets/img/featured3.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Free RN"}</span>
                        <span class="sneaker__preci">{"$149.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                </div>
            </section>
        </>
    }
}