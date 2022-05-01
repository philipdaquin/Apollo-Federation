// pub mod mens_page;
pub mod home;
use crate::router::{home::Home};
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute { 
    #[at("/")]
    Home,



    
}

pub fn switch(routes: &AppRoute) -> Html { 
     match routes { 
        AppRoute::Home => html! { <Home/>},
        // AppRoute::About => html! { <About/>},
    }
}