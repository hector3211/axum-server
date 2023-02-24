use diesel::prelude::*;
use diesel::BelongingToDsl;

use crate::models::NewTodo;
use crate::models::{User,NewUser,Todo};
use crate::schema::todos;
use crate::schema::users;
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
) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let result = diesel::update(users.filter(id.eq(user_id)))
        .set((
            username.eq(user_name),
            hashed_password.eq(user_pw)
        ))
        .get_result(conn)
        .expect("Error updating user");

        Ok(result)
}
//
//
//
//
// Get todos
pub fn get_todos(conn: &mut PgConnection) -> Result<Vec<Todo>, DbError> {
    use crate::schema::todos::dsl::*;

    let data: Vec<Todo> = todos
        .get_results(conn)
        .expect("Error loading all todos!");

    Ok(data)
    
}

// Get Todos by user ID
pub fn get_todos_by_user(
    user_id: i32,
    conn: &mut PgConnection
) -> Result<Vec<Todo>,DbError> {

    let current_user:User = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .get_result(conn)?;


    let todos_for_user: Vec<Todo> = todos::table
        .filter(todos::user_id.eq(current_user.id))
        .select(Todo::as_select())
        .get_results(conn)?;


    Ok(todos_for_user)

}
//
//
// Create Todo
pub fn create_todo_for_user(
    todo_title: String,
    todo_body: String,
    todo_completed: bool,
    the_user_id: i32,
    conn: &mut PgConnection
) -> Result<Vec<Todo>,DbError> {
    use crate::schema::todos::dsl::*;

    let new_todo =  NewTodo {
        title: &todo_title,
        body: &todo_body,
        completed: &todo_completed,
        user_id: &the_user_id,
    };

    let data: Vec<Todo> = diesel::insert_into(todos)
        .values(&new_todo)
        .get_results(conn)
        .expect("Error creating new Todo!");


    Ok(data)
}
//
// Delete Todo
//
// Update Todo
