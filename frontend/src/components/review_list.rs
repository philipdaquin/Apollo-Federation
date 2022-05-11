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

    let count = queried_result.len();

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
                    <div class="box">
                        <div>
                            <div>
                                <h3>{heading.expect("Unable to get the header for User Comment")}</h3>
                            </div>
                        </div>
                        <div>
                            <div class="user-rate">
                                <div class="user-info">
                                    <div class="user-avt">
                                        <img src={media.unwrap_or("https://as1.ftcdn.net/v2/jpg/03/53/11/00/1000_F_353110097_nbpmfn9iHlxef4EDIhXB1tdTD0lcWhG9.jpg".to_string())} alt=""/>
                                    </div>
                                    <div class="user-name">
                                        <span class="name">{username}</span>
                                        <span class="rating">
                                            {format!("Rated: {}", "⭐".repeat(user_rating.unwrap_or(1)))}
                                        </span>
                                    </div>
                                </div>
                            </div>
                            <div class="user-rate-content"> 
                                <p>{body}</p>
                            </div>
                        </div>
                    </div>    
                </> 
            }
        }).collect::<Vec<Html>>();
    
    
    html! {
        <>
            <div>
                <div class="box-header">{format!("{} Reviews", count)}</div>
                // Sort By Review Newest and Top Comments
                <br/>
                {review_list}
            </div>
        </>
    }
}