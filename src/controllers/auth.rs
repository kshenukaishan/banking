use actix_web::{post, Responder};

#[post("/auth/sign-up")]
pub async fn sign_up() -> impl Responder {
    "User sign up"
}

#[post("/auth/login")]
pub async fn sign_in() -> impl Responder {
    "User sign in"
}