extern crate diesel;
pub mod actions;
pub mod models;
pub mod schema;
use axum::{
    routing::{get, post},
    http::StatusCode, Router, extract::{Path, State}, response::IntoResponse, Json,
};
use models::{User, Res};
use tracing::{debug_span, info_span, info, warn, instrument};
use tracing_subscriber::field::debug;
use std::net::SocketAddr;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenvy::dotenv;
use std::env;
use tracing_futures::Instrument;


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
        .route("/", get(get_user))
        .route("/user/:user_name/:user_pw", post(create_user))
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

async fn async_test() -> &'static str {
    info!("handling request");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    warn!("response delayed");

    "hello world"
}

#[instrument]
async fn get_user(
    State(state): State<DbPool>
) -> Result<Json<Vec<User>>,StatusCode> {

    info!("Started Tokio async");
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

    let new_user = tokio::task::spawn_blocking(move || {
        let mut conn = state.get()?;
        actions::create_user(&mut conn, user_name, user_pw)
    })
    .await
    .unwrap();

    Ok((StatusCode::OK,Json(new_user.ok().unwrap())))
}


