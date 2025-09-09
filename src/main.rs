mod controllers;

use actix_web::{App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controllers::auth::sign_up)
            .service(controllers::auth::sign_in)
            .service(controllers::user::create_user)
            .service(controllers::user::get_users)
            .service(controllers::user::users_list)
            .service(controllers::user::update_user)
            .service(controllers::user::delete_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
