use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::{todos,users};

#[derive(
    Debug,
    Selectable,
    Queryable,
    Deserialize,
    Identifiable,
    Serialize,
    Clone,
    PartialEq,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
}


#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashed_password: &'a str,
}


#[derive(
    Debug,
    Selectable,
    Identifiable,
    Queryable,
    Associations,
    Deserialize,
    Serialize,
    Clone,
    PartialEq,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub completed: bool,
    pub user_id: i32,
}


#[derive(Insertable, Serialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub completed: &'a bool,
    pub user_id: &'a i32,
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Res {
    pub message: String,
    pub status: i32,
}

#[derive(Serialize)]
pub struct UsersWithTodos {
    #[serde(flatten)]
    pub user: User,
    pub todos: Vec<Todo>,
}
