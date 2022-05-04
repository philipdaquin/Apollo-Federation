use yew::{prelude::*, function_component, html, Html};
use crate::hooks::{use_query::use_query, UserQuery, user_query};
use crate::models::user::UserInfo;
use crate::components::{
    header::HeaderObject,
    feature::FeatureObject,
    collection::ColletionObject,
    womens::WomensObject,
    newsletter::NewsletterObject,
    productlist::Productlist

};

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
                <Productlist/>
                <HeaderObject/>
                <FeatureObject/>
                <ColletionObject/>
                <WomensObject/>
                <section class="offer section">
                    <div class="offer__container bd-grid">
                        <div class="offer__data">
                            <h3 class="offer__title">{"50%"}</h3>
                            <p class="offer__description">{"in Adidas Superstar sneakers"}</p>
                            <a href="#" class="button">{"Shop Now"}</a>
                        </div>
                        <img src="../../assets/img/offert.png" alt="" class="offer__img"/>            
                    </div>
                </section>
            </section>



            <section class="new section" id="new">
                <h2 class="section-title">{"NEW COLLECTION"}</h2>
                <div class="new__container bd-grid">
                    <div class="new__mens">
                        <img src="../../assets/img/new1.png" alt="" class="new__mens-img"/>
                        <h3 class="new__title">{"Mens Shoes"}</h3>
                        <span class="new__preci">{"From 179.99"}</span>
                        <a href="#" class="button-light">{"View Collection "}<i class="bx bx-right-arrow-alt button-icon"></i></a>
                    </div>
                    <div class="new__sneaker">
                        <div class="new__sneaker-card">
                            <img src="../../assets/img/new2.png" alt="" class="new__sneaker-img"/>
                            <div class="new__sneaker-overlay">
                                <a href="#" class="button">{"Add to Cart"}</a>
                            </div>
                        </div>

                        <div class="new__sneaker-card">
                            <img src="../../assets/img/new3.png" alt="" class="new__sneaker-img"/>
                            <div class="new__sneaker-overlay">
                                <a href="#" class="button">{"Add to Cart"}</a>
                            </div>
                        </div>
                        <div class="new__sneaker-card">
                            <img src="../../assets/img/new4.png" alt="" class="new__sneaker-img"/>
                            <div class="new__sneaker-overlay">
                                <a href="#" class="button">{"Add to Cart"}</a>
                            </div>
                        </div>
                        
                        <div class="new__sneaker-card">
                            <img src="../../assets/img/new5.png" alt="" class="new__sneaker-img"/>
                            <div class="new__sneaker-overlay">
                                <a href="#" class="button">{"Add to Cart"}</a>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <NewsletterObject/>
        </>
    }
}