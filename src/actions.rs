use diesel::{prelude::*, helper_types::FindBy};
use crate::{models::{User,NewUser,Todo,NewTodo}, schema::users};
use diesel::PgConnection;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;


pub fn get_users(conn: &mut PgConnection) -> Result<Vec<User>,DbError> {
    use crate::schema::users::dsl::*;
    let data:Vec<User> = users
        .get_results(conn)
        .expect("Error getting users");
    Ok(data)
}

pub fn create_user(
    conn: &mut PgConnection,
    name:String,
    pw:String
) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let new_user = NewUser {
        username: &name,
        hashed_password: &pw,
    };

    let result = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .expect("Error creating ueser");

    Ok(result)
}

// Delete user
pub fn delete_user(
    user_id: i32,
    conn: &mut PgConnection
) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .expect("Error deleting user");
    Ok(())
}

// Update user
pub fn update_user_info(
    user_name: String,
    user_pw: String,
    user_id: i32,
    conn: &mut PgConnection
) -> Result<(), DbError> {
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(user_id)))
        .set((
            username.eq(user_name),
            hashed_password.eq(user_pw)
        ))
        .execute(conn)
        .expect("Error updating user");

        Ok(())
}
//
//
//

// Get Todos
//
//
// Create Todo
//
// Delete Todo
//
// Update Todo
