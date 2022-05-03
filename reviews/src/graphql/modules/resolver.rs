use super::model::{Review, NewReview};
use diesel::prelude::*;
use crate::schema::review;

pub fn get_reviews(conn: &PgConnection) -> QueryResult<Vec<Review>> { 
    review::table.load(conn)
}

pub fn get_review_by_id(review_id: i32, conn: &PgConnection) -> QueryResult<Review> { 
    review::table.filter(review::id.eq(review_id)).get_result(conn)
}

pub fn get_reviews_by_user(user_id: i32, conn: &PgConnection) -> QueryResult<Vec<Review>> { 
    review::table.filter(review::author_id.eq(user_id)).load(conn)
}

pub fn get_reviews_by_product(product_id: i32, conn: &PgConnection) -> QueryResult<Vec<Review>> { 
    review::table.filter(review::product_id.eq(product_id)).load(conn)
}

pub fn create_review(new_review: NewReview, conn: &PgConnection) -> QueryResult<Review> { 
    diesel::insert_into(review::table)
        .values(new_review)
        .get_result(conn)
}

pub fn delete_review(review_id: i32, conn: &PgConnection) -> QueryResult<bool> { 
    diesel::delete(review::table)
        .filter(review::id.eq(review_id))
        .execute(conn)?;
    Ok(true)
}

pub fn update_review(
    new_review: NewReview, 
    review_id: i32, 
    author_id: i32,
    conn: &PgConnection        
) -> QueryResult<Review> { 
    diesel::update(
        review::table
        .filter(review::author_id.eq(author_id))
        .find(review_id)
    )
    .set(new_review)
    .get_result::<Review>(conn)
}
