use yew::{prelude::*, function_component, html, Html};

#[function_component(NewsletterObject)]
pub fn newsletter_object() -> Html {
    html! {
        <>
            <section class="newsletter section">
                <div class="newsletter__container bd-grid">
                    <div>
                        <h3 class="newletter__title">{"Subscriber And Get "}<br/>{" 10% OFF"}</h3>
                        <p class="newsletter__description">{"Get 10% discount for all products"}</p>
                    </div>
                    <form action="" class="newsletter_subscribe">
                        <input type="text" placeholder="@email.com" class="newsletter_input"/>
                        <a href="#" class="button">{"Subscribe"}</a>
                    </form>
                </div>
            </section>
        </>
    }
}