use yew::{prelude::*, function_component, html, Html};
use crate::components::product_list::{Productlist, ProductListFilter};
#[function_component(FeaturePage)]
pub fn feature_page() -> Html {
    html! {
        <>


            <section class="featured section" id="shop">
                <h2 class="section-title">{"All Products"}</h2>
                <div class="featured__container bd-grid">
                    <Productlist filter={ProductListFilter::All}/>
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