use graphql_client::GraphQLQuery;
use serde_json::json;
use web_sys::HtmlInputElement;
use yew::{prelude::*, function_component, html, Html};
use crate::{hooks::{register_user::{self, NewUserInput, Role}, 
use_query, self, GraphQLResponse}, 
models::user::NewUserRegister, graphql::mutation::register_user_mutation};
use crate::graphql::mutation::RegisterUser;
use yew_hooks::use_async;
use crate::graphql::mutation::register_user_mutation::NewUser;


#[function_component(RegisterUserComponent)]
pub fn register_user_component() -> Html {

    let new_username = use_state(|| "".to_string());
    let new_firstname = use_state(|| "".to_string());
    let new_lastname = use_state(|| "".to_string());
    let new_email = use_state(|| "".to_string());
    let new_password = use_state(|| "".to_string());
    let new_role = use_state(|| "".to_string());
    let register_info = use_state(NewUserRegister::default);
    
    let onsubmit = { 
        let new_username = new_username.clone();
        let new_firstname = new_firstname.clone();
        let new_lastname = new_lastname.clone();
        let new_email = new_email.clone();
        let new_password = new_password.clone();
        let new_role = new_role.clone();


        Callback::from(move |_| { 
            let new_username = new_username.clone();
            let new_firstname = new_firstname.clone();
            let new_lastname = new_lastname.clone();
            let new_email = new_email.clone();
            let new_password = new_password.clone();
            let new_role = new_role.clone();
            wasm_bindgen_futures::spawn_local(async move { 
                let register_info = NewUserRegister {
                    username: (*new_username).clone(), 
                    first_name: (*new_firstname).clone(), 
                    last_name: (*new_lastname).clone(), 
                    email: (*new_email).clone(), 
                    password: (*new_password).clone(), 
                    role: (*new_role).clone() 
                };
                let variables = NewUser::from(&register_info);
                let request_body = RegisterUser::build_query(register_user_mutation::Variables { 
                    new_character: variables
                });
                let request_json = json!(request_body);
                let request = hooks::build_request(&request_json).await;
                if let Ok(response) = request {
                    let json = response.json::<GraphQLResponse<bool>>().await;
                    match json {
                        Ok(_responser) => { ()},
                        Err(_error) => (),
                    }
                } 
            })
        })
    };  

    // let oninput_username = {
    //     let new_username = new_username.clone();
    //     Callback::from(move |input| new_username.set(input))
    // };
    // let oninput_firstname = {
    //     let new_firstname = new_firstname.clone();
    //     Callback::from(move |input| new_firstname.set(input))
    // };
    // let oninput_lastname = {
    //     let new_lastname = new_lastname.clone();
    //     Callback::from(move |input| new_lastname.set(input))
    // };
    // let oninput_email = {
    //     let new_email = new_email.clone();
    //     Callback::from(move |input| new_email.set(input))
    // };
    // let oninput_password = {
    //     let new_password = new_password.clone();
    //     Callback::from(move |input| new_password.set(input))
    // };
    // let oninput_role = {
    //     let new_role = new_role.clone();
    //     Callback::from(move |input| new_role.set(input))
    // };



    let oninput_username = { 
        let new_username = new_username.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_username).clone();
            info = input.value();
            new_username.set(info)
        })
    };
    let oninput_firstname = { 
        let new_firstname = new_firstname.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_firstname).clone();
            info = input.value();
            new_firstname.set(info)
        })
    };
    let oninput_lastname = { 
        let new_lastname = new_lastname.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_lastname).clone();
            info = input.value();
            new_lastname.set(info)
        })
    };
    let oninput_email = { 
        let new_email = new_email.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_email).clone();
            info = input.value();
            new_email.set(info)
        })
    };
    let oninput_password = { 
        let new_password = new_password.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_password).clone();
            info = input.value();
            new_password.set(info)
        })
    };
    let oninput_role = { 
        let new_role = new_role.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*new_role).clone();
            info = input.value();
            new_role.set(info)
        })
    };






    return html! {
        <>
            <div>
                <div>
                    <h3>{"Register Page"}</h3>
                    <p>{"Register Now!"}</p>
                    <form {onsubmit} >
                        <fieldset>
                            <input 
                                value={(*new_username).clone()}
                                oninput={oninput_username}
                                placeholder="Username"
                                type="text"
                            />
                            <input 
                                value={(*new_firstname).clone()}
                                oninput={oninput_firstname}
                                placeholder="firstname"
                                type="text"
                            />
                            <input 
                                value={(*new_lastname).clone()}
                                oninput={oninput_lastname}
                                placeholder="lastname"
                                type="text"
                            />
                            <input 
                                value={(*new_email).clone()}
                                oninput={oninput_email}
                                placeholder="email"
                                type="text"
                            />
                            <input 
                                value={(*new_password).clone()}
                                oninput={oninput_password}
                                placeholder="password"
                                type="text"
                            />
                            <input 
                                value={(*new_role).clone()}
                                oninput={oninput_role}
                                placeholder="role"
                                type="text"
                            />
                        </fieldset>
                    </form>
                    <button type="submit" disabled=false>{"Submit ⭐⭐"}</button>
                </div>
            </div>
        </>
    }
}