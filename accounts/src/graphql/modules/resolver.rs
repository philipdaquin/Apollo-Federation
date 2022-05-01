use super::model::{User, NewUser};
use diesel::prelude::*;
use crate::schema::users;


pub fn get_user_by_id(id: i32, conn: &PgConnection) -> QueryResult<User> {
    users::table.filter(users::id.eq(id)).first(conn)
}

pub fn get_user_by_name(name: String, conn: &PgConnection) -> QueryResult<User> { 
    users::table    
        .filter(users::username.eq(name))
        .first(conn)
}

pub fn get_all_users(conn: &PgConnection) -> QueryResult<Vec<User>> { 
    users::table.load(conn)
}

pub fn create_user(new_user: NewUser, conn: &PgConnection) -> QueryResult<User> { 
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
}

pub fn update_user_details(user_id: i32, new_user: NewUser, conn: &PgConnection) -> QueryResult<User> { 
    diesel::update(users::table)
        .filter(users::id.eq(user_id))
        .set(new_user)
        .get_result(conn)
}

pub fn delete_user(user_id: i32, conn: &PgConnection) -> QueryResult<bool> { 
    diesel::delete(users::table)
        .filter(users::id.eq(user_id))
        .execute(conn)?;
    Ok(true)
}
