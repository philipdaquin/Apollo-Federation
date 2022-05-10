use yew::{prelude::*, function_component, html, Html};
use crate::router::AppRoute;
use yew_router::prelude::*;


#[function_component(HeaderObject)]
pub fn header_component() -> Html {
    html! {
        <>
            <section class="home" id="home">
                <div class="home__container bd-grid">
                    <div class="home__sneaker">
                        <div class="home__shape"></div>
                        <img src="../../assets/img/imghome.png" alt="" class="home__img"/>
                    </div>
                    <div class="home__data">
                        <span class="home__new">{"New in"}</span>
                        <h1 class="home__title">{"YEEZY BOOST"} <br/> {"SPLY - 350"}</h1>
                        <p class="home__description">{"Explore the new collections of sneakers"}</p>
                        
                        <Link<AppRoute> to={AppRoute::FeaturePage} classes="button">
                            {"Explore Now"}
                        </Link<AppRoute>>
                    </div>
                </div>
            </section>
        </>
    }
}