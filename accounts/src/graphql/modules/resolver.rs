use super::model::User;
use diesel::prelude::*;
use crate::schema::users;


pub fn get_user_by_id(id: i32, conn: &PgConnection) -> QueryResult<User> {
    users::table.filter(users::id.eq(id)).first(conn)
}

pub fn get_all_users(conn: &PgConnection) -> QueryResult<Vec<User>> { 
    users::table.load(conn)
}