#[macro_use]
extern crate diesel;
extern crate dotenv;

use warp::Filter;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use std::sync::{Arc, Mutex};

mod schema;
mod models;

use models::{User, NewUser};
use schema::users::dsl::*;

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = Arc::new(Mutex::new(establish_connection(&database_url)));

    // Serve static files from the "static" directory
    let static_files = warp::fs::dir("static");

    // Define login route
    let login_route = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_db(connection.clone()))
        .map(|login: LoginRequest, conn: Arc<Mutex<SqliteConnection>>| {
            let mut conn = conn.lock().unwrap();
            let user = users
                .filter(username.eq(&login.username))
                .first::<User>(&mut *conn)
                .optional()
                .expect("Error loading user");

            if let Some(user) = user {
                if user.password == login.password {
                    warp::reply::json(&"Login successful")
                } else {
                    warp::reply::json(&"Invalid username or password")
                }
            } else {
                warp::reply::json(&"Invalid username or password")
            }
        });

    // Define register route
    let register_route = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(with_db(connection.clone()))
        .map(|register: RegisterRequest, conn: Arc<Mutex<SqliteConnection>>| {
            let mut conn = conn.lock().unwrap();
            let new_user = NewUser {
                username: register.username,
                password: register.password,
            };

            diesel::insert_into(users)
                .values(&new_user)
                .execute(&mut *conn)
                .expect("Error saving new user");

            warp::reply::json(&"User registered successfully")
        });

    // Combine routes
    let routes = static_files.or(login_route).or(register_route);

    // Start the warp server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn with_db(
    conn: Arc<Mutex<SqliteConnection>>,
) -> impl Filter<Extract = (Arc<Mutex<SqliteConnection>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || conn.clone())
}
