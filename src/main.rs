use actix_web::{App, HttpServer, web};
use http_handler::{add_user, add_vehicle, get_my_vehicles, get_users, get_vehicles};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_url = env::var("HOST_URL").expect("HOST_URL must be set");
    let api_port = env::var("HOST_PORT").expect("HOST_PORT must be set");

    // Make sql connection, set db tables
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    // Create a users table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            mobile VARCHAR NOT NULL,
        )",
    )
    .execute(&pool)
    .await
    .expect("Failed to create db table User");

    // Start listeneing for requests
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::get().to(get_users))
            .route("/vehicles", web::get().to(get_vehicles))
            .route("/my_vehicles", web::get().to(get_my_vehicles))
            .route("/add_user", web::get().to(add_user))
            .route("/add_vehicle", web::get().to(add_vehicle))
    })
    .bind((api_url, api_port.parse::<u16>().unwrap()))?
    .run()
    .await
}
