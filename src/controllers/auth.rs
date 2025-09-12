use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use serde_json::json;
use sqlx::MySqlPool;
use crate::AppState;
use crate::databse::user_query::{create_user, has_email_already};
use crate::models::sign_in_request::SignInRequest;
use crate::models::sign_up_request::SignUpRequest;

#[post("/auth/sign-up")] // Registration
pub async fn sign_up(state: Data<AppState>, user: Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().await;

    if has_email_already(&db, &user.email).await {
        return HttpResponse::UnprocessableEntity().json(json!({
            "status": "error",
            "message": "User already exists"
        }).to_string())
    }

    create_user(&db, &user).await;

    HttpResponse::Created().json(json!({
        "status": "success",
        "message": "User successfully signed up"
    }).to_string())
}

#[post("/auth/login")] // Email, Password
pub async fn sign_in(user: Json<SignInRequest>) -> impl Responder {
    HttpResponse::Ok().json(SignInRequest {
        email: user.email.clone(),
        password: user.password.clone(),
    })
}