use yew::{prelude::*, function_component, html, Html};

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ProfileProps { 
    #[prop_or_default]
    pub id: i32,
    #[prop_or_default]
    pub username: String
}


#[function_component(ProfilePage)]
pub fn profile_page(ProfileProps {id, username}: &ProfileProps) -> Html {
    html! {
        <>
            <section>
                
            </section>
        </>
    }
}