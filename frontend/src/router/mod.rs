// pub mod mens_page;
pub mod home;
pub mod featurepage;

use crate::router::{home::Home, featurepage::FeaturePage};
use yew_router::prelude::*;
use yew::prelude::*;


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute { 
    #[at("/")]
    Home,
    #[at("/feature_all")]
    FeaturePage,



    
}

pub fn switch(routes: &AppRoute) -> Html { 
     match routes { 
        AppRoute::Home => html! { <Home/>},
        AppRoute::FeaturePage => html! { <FeaturePage/>},
    }
}