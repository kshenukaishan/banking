mod controllers;
mod models;
mod databse;

use actix_web::{web, App, HttpServer, Responder};
use sqlx::MySqlPool;
use tokio::sync::Mutex;

pub struct AppState {
    pub db: Mutex<MySqlPool>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();

    let state = web::Data::new(AppState {
        db: Mutex::new(MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await.unwrap())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(controllers::auth::sign_up)
            .service(controllers::auth::sign_in)
            // .service(controllers::user::create_user)
            .service(controllers::user::get_users)
            .service(controllers::user::users_list)
            .service(controllers::user::update_user)
            .service(controllers::user::delete_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
