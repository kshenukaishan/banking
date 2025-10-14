use crate::database::user_query::{check_user_exist, create_user, has_email_already};
use crate::models::claims::Claims;
use crate::models::sign_in_request::SignInRequest;
use crate::models::sign_up_request::SignUpRequest;
use crate::AppState;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse, Responder};
use jsonwebtoken::{EncodingKey, Header};
use serde_json::json;
use std::time::SystemTime;

#[post("/sign-up")] // Registration
pub async fn sign_up(state: Data<AppState>, user: Json<SignUpRequest>) -> impl Responder {
    let db = state.db.lock().unwrap();

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

#[post("/login")] // Email, Password
pub async fn sign_in(data: Json<SignInRequest>, state: Data<AppState>) -> impl Responder {
    let db = state.db.lock().unwrap();

    let Some(user) = check_user_exist(&db, &data.email).await else {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "User does not exist"
        }))
    };

    if !bcrypt::verify(&data.password, &user.password).unwrap() {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid password"
        }))
    }

    let claims = Claims {
        sub: user.id,
        role: "User".to_string(),
        exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + 4 * 60 * 60,
    };

    let token = match jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.secret_key.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Token generation failed"
        }))
        }
    };

    HttpResponse::Ok().json(json!({
        "status": "success",
        "token": token,
    }))
}