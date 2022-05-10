use yew::{prelude::*, function_component, html, Html};


#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ProfileProps { 
    #[prop_or_default]
    pub user_id: i32
}


#[function_component(ProfilePage)]
pub fn profile_page(ProfileProps {user_id}: &ProfileProps) -> Html {
    html! {
        <>
        
        

        </>
    }
}