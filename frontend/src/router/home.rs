use yew::{prelude::*, function_component, html, Html};
use crate::hooks::{use_query::use_query, UserQuery, user_query};
use crate::models::user::UserInfo;


#[function_component(Home)]
pub fn home_page() -> Html {
    let variables = user_query::Variables {};
    let get_all_users  = use_query::<UserQuery>(variables);
    
    if get_all_users.data.is_none() { 
        return html! {
            <>
                <h1>{"Query Failed!"}</h1>
            </>
        }
    }

    let queried_results: Vec<UserInfo> = get_all_users  
        .data
        .unwrap()
        .get_all_users
        .iter()
        .map(|user| UserInfo::from(user))
        .collect();
    
    let user_info = queried_results
        .iter()
        .map(|info| { 
            return html! { 
                <>
                    <div>
                        <h1>{info.id}</h1>
                        <h1>{info.first_name.clone()}</h1>
                        <h3>{info.username.clone()}</h3>
                        <h3>{info.password.clone()}</h3>
                        <h3>{info.email.clone()}</h3>
                        <h3>{info.joined_at.clone()}</h3>
                    </div>
                </>
            }
        }).collect::<Vec<Html>>();

    html! {
        <>
            <section>
                {user_info}
            </section>
        </>
    }
}