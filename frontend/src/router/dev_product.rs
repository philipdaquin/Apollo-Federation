use yew::{prelude::*, function_component, html, Html};

#[function_component(CreateProduct)]
pub fn create_product() -> Html {
    html! {
        <>
            <section class="featured section" id="shop">
                <h2 class="section-title">{"All Products"}</h2>
                <div class="featured__container bd-grid">
                    <p>{"Hello"}</p>                
                
                
                </div>
            </section>
        </>
    }
}