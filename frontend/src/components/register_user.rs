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


    let register_info = use_state(NewUserRegister::default);
    let onsubmit = { 
        let register_info = register_info.clone();

        Callback::from(move |_| { 
            let register_info = register_info.clone();

            wasm_bindgen_futures::spawn_local(async move { 
                let register_info = (*register_info).clone();
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

    let oninput_username = { 
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info)
        })
    };
    let oninput_firstname = { 
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.first_name = input.value();
            register_info.set(info)
        })
    };
    let oninput_lastname = { 
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.last_name = input.value();
            register_info.set(info)
        })
    };
    let oninput_email = { 
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info)
        })
    };
    let oninput_password = { 
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info)
        })
    };
    let oninput_role = { 
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.role = input.value();
            register_info.set(info)
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
                                value={register_info.username.clone()}
                                oninput={oninput_username}
                                placeholder="Username"
                                type="text"
                            />
                            <input 
                                value={register_info.first_name.clone()}
                                oninput={oninput_firstname}
                                placeholder="firstname"
                                type="text"
                            />
                            <input 
                                value={register_info.last_name.clone()}
                                oninput={oninput_lastname}
                                placeholder="lastname"
                                type="text"
                            />
                            <input 
                                value={register_info.email.clone()}
                                oninput={oninput_email}
                                placeholder="email"
                                type="text"
                            />
                            <input 
                                value={register_info.password.clone()}
                                oninput={oninput_password}
                                placeholder="password"
                                type="text"
                            />
                            <input 
                                value={register_info.role.clone()}
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