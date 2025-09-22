use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{web, Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use crate::AppState;
use crate::models::claims::Claims;

pub async fn verify_token(req: ServiceRequest, next: Next<BoxBody>) -> Result<ServiceResponse, Error> {
    let auth_header = req.headers().get("Authorization").ok_or_else(|| {
        ErrorUnauthorized(json!({
            "status": "UNAUTHORIZED",
            "message": "Authorization header missing"
        }))
    })?;

    let auth_string = auth_header.to_str().map_err(|_| {
        ErrorUnauthorized(json!({
            "status": "UNAUTHORIZED",
            "message": "Invalid Authorization header"
        }))
    })?;

    if !auth_string.starts_with("Bearer ") {
        return Err(ErrorUnauthorized(json!({
            "status": "UNAUTHORIZED",
            "message": "Invalid Authorization header"
        })))
    }

    let token = auth_string.strip_prefix("Bearer ").unwrap();
    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let key = DecodingKey::from_secret(&state.secret_key.as_bytes());

    match decode::<Claims>(token, &key, &Validation::default()) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims.sub);
            next.call(req).await
        }
        Err(_) => {
            Err(ErrorUnauthorized(json!({
                "status": "UNAUTHORIZED",
                "message": "Invalid token"
            })))
        }
    }
}