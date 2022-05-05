use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{AppRoute, switch};

#[function_component(Navbar)]
pub fn navbar() -> Html { 
    html! {
        <>
            <header class="l-header" id="header">
                <nav class="nav bd-grid">
                    <div class="nav__toggle" id="nav-toggle">
                        <i class="bx bxs-grid"></i>
                    </div>
                    <Link<AppRoute> to={AppRoute::Home} classes={"nav__link"}>
                        <a href="#" class="nav__logo">{"Apollo 🚀"}</a>
                    </Link<AppRoute>>
                    <div class="nav__menu" id="nav-menu">
                        <ul class="nav__list">
                            <li><Link<AppRoute> to={AppRoute::Home} classes={"nav__link"}>{"Home"}</Link<AppRoute>></li>
                            <li><Link<AppRoute> to={AppRoute::FeaturePage} classes={"nav__link"}>{"All Products"}</Link<AppRoute>></li>
                            // <Link<AppRoute> to={AppRoute::Feature} classes={"nav__link"}>{"Feature"}</Link<AppRoute>>
                            // <Link<AppRoute> to={AppRoute::Mens} classes={"nav__link"}>{"Mens"}</Link<AppRoute>>
                            // <Link<AppRoute> to={AppRoute::Womens} classes={"nav__link"}>{"Womens"}</Link<AppRoute>>
                            // <Link<AppRoute> to={AppRoute::New} classes={"nav__link"}>{"New"}</Link<AppRoute>>
                            // <Link<AppRoute> to={AppRoute::Shop} classes={"nav__link"}>{"Shop"}</Link<AppRoute>>
                        </ul>
                    </div>
                    <div class="nav__shop">
                        <i class="bx bx-shopping-bag"></i>
                    </div>
                </nav>
            </header>
        </>
    }
}
