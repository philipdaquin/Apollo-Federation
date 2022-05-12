use web_sys::HtmlInputElement;
use yew::{prelude::*, function_component, html, Html};
use crate::{hooks::{register_user,  RegisterUser, use_query}, models::user::NewUserRegister};


#[function_component(RegisterUserComponent)]
pub fn register_user_component() -> Html {
    

    let register_info = use_state(NewUserRegister::default);
    let oninput_username = { 
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| { 
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.clone();
        })
    };


    let on_submit = {
        let NewUserRegister {
            username, first_name,last_name, email, role
        } = register_info.clone();

        Callback::from(move |_| { 
            wasm_bindgen_futures::spawn_local(async move { 
                let submit_username = (*username).clone();

                let variables = regisrer_user::Variables { 
                    username: submit_username,
                };

                let request_biody = RegisterUser::build_query(variables);
                let request_json = &json!(request_body);
                let request = hooks::build_request(request_json).await;
                if let Ok(res) = request { 
                    let json = res.json::<GraphQLResponse<bool>>().await;
                    match json { 
                        Ok(i) => (),
                        Err(e) => ()
                    }
                }
                
            })
        })
    };


    // let variables = register_user::Variables {
    //     new_user: 
    // };
    // let query = use_query::<RegisterUser>(variables);
    
    return html! {
        <>
            <div>
                <div>
                    <h3>{"Register Page"}</h3>
                    <p>{"Register Now!"}</p>
                    <form action="">
                        <fieldset>
                            <input 
                                value={register_info.username.clone()}
                                oninput={oninput_username}
                                placeholder="Username"
                                type="text"
                            />
                        </fieldset>
                    </form>

                </div>
            </div>
        </>
    }
}