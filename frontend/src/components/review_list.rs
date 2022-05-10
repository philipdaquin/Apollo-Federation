use yew::{prelude::*, function_component, html, Html};
use crate::hooks::get_reviews_of_any_product_id;
use crate::hooks::{GetReviewsOfAnyProductId, use_query};
use crate::models::reviews::{ReviewResponseData, AuthorData};


#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ReviewProps { 
    pub product_id: i32
}

#[function_component(ReviewList)]
pub fn review_list(ReviewProps {product_id}: &ReviewProps) -> Html {
    
    let variables = get_reviews_of_any_product_id::Variables {
        product_id: product_id.to_string()
    };
    let get_reviews = use_query::<GetReviewsOfAnyProductId>(variables);

    if get_reviews.data.is_none() { 
        return html!  {
            <>
                <h2>{"Failed to get all Product Reviews"}</h2>
            </>
        }
    }

    let queried_result: Vec<ReviewResponseData> = get_reviews
        .data
        .expect("Theres no data under here ")
        .get_reviews_of_any_product_id
        .iter()
        .map(|f| ReviewResponseData::from(f))
        .collect();


    let review_list = queried_result 
        .iter()
        .map(|f| { 
            let ReviewResponseData { 
                id, 
                body, 
                heading, 
                media, 
                is_edited, 
                user_rating,
                author
            } = f.clone();

            let AuthorData  {
                id,
                username, 
                firstname,
                lastname
            } = author.clone();


            return html! {
                <>
                    <div>
                        <div>
                            <div>
                                <h1>{heading.expect("")}</h1>
                            </div>
                            <h1>{author.id}</h1>
                            <h2>{username}</h2>
                            <h3>{format!("{}{}", firstname, lastname)}</h3>
                        </div>
                        <p>{format!(" User Rating {}", user_rating.unwrap_or(0))}</p>
                        <p>{body}</p>
                    </div>    
                          
                </> 
            }
        }).collect::<Vec<Html>>();
    
    
    html! {
        <>
            {review_list}
        </>
    }
}