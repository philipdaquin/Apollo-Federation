use yew::{prelude::*, function_component, html, Html};
use crate::hooks::get_reviews_of_any_product_id;
use crate::hooks::{GetReviewsOfAnyProductId, use_query};
use crate::models::reviews::ReviewResponseData;
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
        .unwrap()
        .get_reviews_of_any_product_id
        .iter()
        .map(|f| ReviewResponseData::from(f))
        .collect();


    let review_list = queried_result 
        .iter()
        .map(|f| { 
            let ReviewResponseData { 
                id, body, heading, media, is_edited, user_rating
            } = f.clone();
            return html! {
                <>
                    <div class="user-rate">
                        <div class="user-info">
                            <div class="user-avt">
                                <img src="https://imageio.forbes.com/specials-images/imageserve/61578d13c3a591704133bc40/Header-TrumpL-1x1/0x0.gif?format=gif&height=1080&width=1080&fit=crop" alt=""/>
                            </div>
                            <div class="user-name">
                            <span class="name">{""}</span>
                                <span class="rating">

                                    {
                                        format!("{}", user_rating.unwrap())
                                    }
                                    
                                    <i class="bx bxs-star"></i>
                                    <i class="bx bxs-star"></i>
                                    <i class="bx bxs-star"></i>
                                    <i class="bx bxs-star"></i>
                                </span>
                            </div>
                        </div>
                        <div class="user-rate-content">
                            <h3>{heading}</h3>
                            <p>{body}</p>
                            <img src={media.clone().unwrap()} alt=""/>
                        </div>
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