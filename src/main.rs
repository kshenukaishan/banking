mod controllers;
mod models;
mod database;

mod middleware;
mod utils;

use std::sync::Mutex;
use actix_web::middleware::from_fn;
use actix_web::{web, App, HttpServer, Responder};
use sqlx::MySqlPool;

pub struct AppState {
    pub db: Mutex<MySqlPool>,
    pub secret_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv::dotenv().ok();

    let state = web::Data::new(AppState {
        db: Mutex::new(MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await.unwrap()),
        secret_key: std::env::var("ENCODE_KEY").unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/auth")
                .service(controllers::auth::sign_up)
                .service(controllers::auth::sign_in)
            )
            .service(
                web::scope("/api")
                    .wrap(from_fn(middleware::auth_verify::verify_token))
                    .service(controllers::user::get_user)
                    // .service(controllers::user::users_list)
                    .service(controllers::user::update)
                    // .service(controllers::user::delete_user)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
