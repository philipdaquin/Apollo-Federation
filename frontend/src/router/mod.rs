// pub mod mens_page;
pub mod home;
pub mod featurepage;
pub mod product_detail;

use crate::router::{
    home::Home, 
    featurepage::FeaturePage,
    product_detail::ProductDetail

};
use yew_router::prelude::*;
use yew::prelude::*;


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute { 
    #[at("/")]
    Home,

    #[at("/feature_all")]
    FeaturePage,
    
    #[at("/product/:id")]
    ProductPage { id:  i32 },
    
}

pub fn switch(routes: &AppRoute) -> Html { 
     match routes { 
        AppRoute::Home => html! { <Home/>},
        AppRoute::FeaturePage => html! { <FeaturePage/>},
        AppRoute::ProductPage { id } => html! { <ProductDetail id={*id}/>}
    }
}