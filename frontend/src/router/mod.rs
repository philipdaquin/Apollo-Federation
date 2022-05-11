// pub mod mens_page;
pub mod home;
pub mod featurepage;
pub mod product_detail;
pub mod profile_page;
pub mod dev_product;

use crate::router::{
    home::Home, 
    featurepage::FeaturePage,
    product_detail::ProductDetail,
    profile_page::ProfilePage,
    dev_product::CreateProduct
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

    #[at("/profile/:id?/:username")]
    ProfilePage { 
        id: i32, 
        username: String,
    },
    #[at("/developer")]
    Developer
    
}

pub fn switch(routes: &AppRoute) -> Html { 
     match routes { 
        AppRoute::Home => html! { <Home/>},
        AppRoute::FeaturePage => html! { <FeaturePage/>},
        AppRoute::ProductPage { id } => html! { <ProductDetail id={*id}/>},
        AppRoute::ProfilePage { 
            id, 
            username
        } => html! { <ProfilePage id={*id} username={username.clone()} />},
        AppRoute::Developer => html! { <CreateProduct/>}
        
    }
}