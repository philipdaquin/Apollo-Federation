use yew::{prelude::*, function_component, html, Html};

#[derive(Properties, Clone, PartialEq)]
pub struct ProductProps { 
    #[prop_or_default]
    pub id: i32
}

#[function_component(ProductDetail)]
pub fn product_detail(ProductProps {id}: &ProductProps ) -> Html {
   
    html! {
        <>
        </>
    }
}