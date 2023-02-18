use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::{todos,users};
use chrono::NaiveDateTime;

#[derive(Debug,Queryable,Deserialize,Identifiable,Serialize,Clone)]
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


#[derive(Debug,Identifiable,Queryable,Associations,Deserialize,Serialize,Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}


#[derive(Insertable, Serialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub completed: &'a bool,
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Res {
    pub message: String,
    pub status: i32,
}
