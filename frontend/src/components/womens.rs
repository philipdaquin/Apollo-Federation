use yew::{prelude::*, function_component, html, Html};

#[function_component(WomensObject)]
pub fn woments_object() -> Html {
    html! {
        <>
            <section class="women section" id="women">
                <h2 class="section-title">{"WOMEN SNEAKERS"}</h2>
                <div class="women__container bd-grid">
                    <article class="sneaker">
                        <img src="../../assets/img/women1.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Free TN"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>

                    <article class="sneaker ">
                        <img src="../../assets/img/women2.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Free TR"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>

                    <article class="sneaker">
                        <img src="../../assets/img/women3.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike GS Pink"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>

                    <article class="sneaker">
                        <img src="../../assets/img/women1.png" alt="" class="sneaker__img"/>
                        <span class="sneaker__name">{"Nike Get5"}</span>
                        <span class="sneaker__preci">{"$129.99"}</span>
                        <a href="" class="button-light">{"Add to Cart "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </article>
                </div>
            </section>
        </>
    }
}