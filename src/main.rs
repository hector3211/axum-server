use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, extract::Path,
};
use serde::{Deserialize, Serialize};

use std::net::SocketAddr;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use dotenvy::dotenv;
use std::env;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL provided!");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool!");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/users/:user_id/:user_name", get(user_info))
        .with_state(pool);

        

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize,Serialize)]
struct SayName {
    id: i32,
    name: String,
}

async fn user_info(
    Path((user_id,user_name)): Path<(i32,String)>
) -> Result<Json<SayName>, StatusCode> {
    let user = SayName {
        id: user_id,
        name: user_name,
    };

    Ok(Json(user))
}


// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}


// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
