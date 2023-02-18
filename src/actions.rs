use diesel::prelude::*;
use crate::models::{User,NewUser,Todo,NewTodo};
use diesel::PgConnection;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;


pub fn get_users(conn: &mut PgConnection) -> Result<Vec<User>,DbError> {
    use crate::schema::users::dsl::*;
    let data:Vec<User> = users
        .load(conn)
        .expect("Error getting users");
    Ok(data)
}

pub fn create_user(
    conn: &mut PgConnection,
    name:String,
    pw:String
) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;
    let new_user = NewUser {
        username: &name,
        hashed_password: &pw,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error creating ueser");

    Ok(())
}
