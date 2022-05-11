use yew::{prelude::*, function_component, html, Html};
use crate::{hooks::{register_user,  RegisterUser, use_query}, models::user::NewUserRegister};



#[function_component(RegisterUserComponent)]
pub fn register_user_component() -> Html {
    

    let register_info = use_state(|| NewUserRegister::default);
    let oninput_username = { 
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.clone();
        })
    };


    let variables = register_user::Variables {
        new_user: register_info
    };
    let query = use_query::<RegisterUser>(variables);
    
    
    return html! {
        <>
            <div>
                <div>
                    <h3>{"Register Page"}</h3>
                    <p>{"Register Now!"}</p>
                </div>
            </div>
        </>
    }
}