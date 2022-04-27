use super::model::Review;
use diesel::prelude::*;
use crate::schema::review;

pub fn get_reviews(conn: &PgConnection) -> QueryResult<Vec<Review>> { 
    review::table.load(conn)
}

pub fn get_reviews_by_user(user_id: i32, conn: &PgConnection) -> QueryResult<Vec<Review>> { 
    review::table.filter(review::author_id.eq(user_id)).load(conn)
}

pub fn get_reviews_by_product(product_id: i32, conn: &PgConnection) -> QueryResult<Vec<Review>> { 
    review::table.filter(review::product_id.eq(product_id)).load(conn)
}
