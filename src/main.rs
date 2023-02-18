extern crate diesel;
pub mod actions;
pub mod models;
pub mod schema;
use axum::{
    routing::{get, post},
    http::StatusCode, Router, extract::{Path, State}, response::IntoResponse, Json,
};
use models::{User, Res};
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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL provided!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool!");


    let app = Router::new()
        .route("/", get(get_user))
        .route("/user/:user_name/:user_pw", post(create_user))
        .with_state(pool.clone());


    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user(
    State(state): State<DbPool>
) -> Result<Json<Vec<User>>,StatusCode> {

    let users = tokio::task::spawn_blocking(move ||{
        let mut conn = state.get()?;
        actions::get_users(&mut conn)
    })
    .await
    .unwrap();

    if let Ok(users) = users {
        Ok(Json(users))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}


async fn create_user(
    Path((user_name,user_pw)): Path<(String,String)>,
    State(state): State<DbPool>
) -> Result<impl IntoResponse,StatusCode> {

    let insert = tokio::task::spawn_blocking(move || {
        let mut conn = state.get()?;
        actions::create_user(&mut conn, user_name, user_pw)
    })
    .await
    .unwrap();

    if let Ok(insert) = insert {
        let message = Res {
            message: "Successfully made a new user!".to_string(),
            status: 200,
        };
        Ok((StatusCode::OK,Json(message)))
    } else {
        let message = Res {
            message: "Error creating new user".to_string(),
            status: 404,
        };
        Ok((StatusCode::BAD_REQUEST,Json(message)))
    }
}


