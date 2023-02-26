extern crate diesel;
pub mod actions;
pub mod models;
pub mod schema;
use axum::{
    routing::{get, post,put,delete},
    http::StatusCode, Router, extract::{Path, State}, Json,
};
use models::{User, Todo, UsersWithTodos};
use tracing::{info, warn, instrument};
use std::net::SocketAddr;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenvy::dotenv;
use std::env;


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL provided!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool!");


    let app = Router::new()
        .route("/test", get(async_test))
        .route("/users", get(get_users))
        .route("/users/:user_name/:user_pw", post(create_user))
        .route("/users/:user_name/:user_pw/:user_id", put(update_user))
        .route("/users/user/:user_id", delete(delete_user))
        .route("/todos", get(get_todos))
        .route("/todos/:todo_title/:todo_body/:todo_completed/:the_user_id", post(create_user_todo))
        .route("/user/todos", get(all_users_with_todos))
        .with_state(pool.clone());


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn all_users_with_todos(
    State(state): State<DbPool>
) -> Result<Json<Vec<UsersWithTodos>>,StatusCode> {
    let data = tokio::task::spawn_blocking(move || {
        let mut conn = state.get()?;
        actions::get_users_todos(&mut conn)
    })
    .await
    .unwrap();

    Ok(Json(data.ok().unwrap()))
}

async fn async_test() -> &'static str {
    info!("handling request");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    warn!("response delayed");

    "hello world"
}


#[instrument]
async fn get_users(
    State(state): State<DbPool>
) -> Result<Json<Vec<User>>,StatusCode> {

    info!("Started Tokio async for get_users");
    let users = tokio::task::spawn_blocking(move ||{
        let mut conn = state.get()?;
        actions::get_users(&mut conn)
    })
    .await
    .unwrap();
    warn!("Ended Tokio async");

    if let Ok(users) = users {
        Ok(Json(users))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}

#[instrument]
async fn create_user(
    Path((user_name,user_pw)): Path<(String,String)>,
    State(state): State<DbPool>
) -> Result<(StatusCode,Json<User>),StatusCode> {

    info!("Started Tokio async for create_user");
    let new_user = tokio::task::spawn_blocking(move || {
        let mut conn = state.get()?;
        actions::create_user(&mut conn, user_name, user_pw)
    })
    .await
    .unwrap();
    warn!("Ended Tokio async for create_user");

    Ok((StatusCode::OK,Json(new_user.ok().unwrap())))
}

#[instrument]
async fn update_user(
    Path((user_name,user_pw,user_id)): Path<(String,String,i32)>,
    State(state): State<DbPool>
) -> Result<(StatusCode, Json<User>), StatusCode> {
    info!("Started Tokio async for update_user");
    let update_user = tokio::task::spawn_blocking(move || {
        let mut conn = state.get()?;
        actions::update_user_info(user_name, user_pw, user_id, &mut conn)
    })
    .await
    .unwrap();
    warn!("Ended Tokio async for update_user");

    Ok((StatusCode::OK,Json(update_user.ok().unwrap())))
}

#[instrument]
async fn delete_user(
    Path(user_id): Path<i32>,
    State(state): State<DbPool>
) -> Result<StatusCode,StatusCode> {
    info!("Started Tokio async for delete_user");
    let _deleted_user = tokio::task::spawn_blocking(move || {
        let mut conn = state.get()?;
        actions::delete_user(user_id, &mut conn)
    })
    .await
    .unwrap();
    warn!("Ended Tokio async for delete_user");

    Ok(StatusCode::OK)
}

#[instrument]
async fn get_todos(
    State(state): State<DbPool>
) -> Result<Json<Vec<Todo>>,StatusCode> {
    info!("Started Tokio async for getting all todos!");
    let all_todos = tokio::task::spawn_blocking(move || {
        let mut conn = state.get()?;
        actions::get_todos(&mut conn)
    })
    .await
    .unwrap();
    warn!("Ended Tokio async fo getting all todos!");

    Ok(Json(all_todos.ok().unwrap()))
}

#[instrument]
async fn create_user_todo(
    Path((todo_title,todo_body,todo_completed,the_user_id)): Path<(String,String,bool,i32)>,
    State(state): State<DbPool>
) -> Result<Json<Vec<Todo>>,StatusCode> {
    info!("User Todo async fn started!");
    let todos = tokio::task::spawn_blocking(move || {
        let mut conn = state.get()?;
        actions::create_todo_for_user(todo_title,todo_body,todo_completed,the_user_id,&mut conn)
    })
    .await
    .unwrap();
    warn!("User Todo async fn ended!");

    
    Ok(Json(todos.ok().unwrap()))
}

