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
                            <li class="nav__item"><Link<AppRoute> to={AppRoute::Home} classes={"nav__link"}>{"Home"}</Link<AppRoute>></li>
                            <li class="nav__item"><Link<AppRoute> to={AppRoute::FeaturePage} classes={"nav__link"}>{"Featured"}</Link<AppRoute>></li>
                            <li class="nav__item"><Link<AppRoute> to={AppRoute::FeaturePage} classes={"nav__link"}>{"Women"}</Link<AppRoute>></li>
                            <li class="nav__item"><Link<AppRoute> to={AppRoute::FeaturePage} classes={"nav__link"}>{"New"}</Link<AppRoute>></li>
                            <li class="nav__item"><Link<AppRoute> to={AppRoute::FeaturePage} classes={"nav__link"}>{"Shop"}</Link<AppRoute>></li>
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
