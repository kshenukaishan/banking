use crate::{database, AppState};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::account::Account;
use crate::utils::get_user_id;

#[get("/list")]
pub async fn account_list(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    "Accounts get"
}

#[post("/new")]
pub async fn create_account_controller(
    state: web::Data<AppState>,
    account: web::Json<Account>,   // 👈 JSON body
) -> impl Responder {
    let pool: &MySqlPool = &state.db;   // borrow pool from AppState

    let success = database::account_query::create_account(pool, account).await;

    if success {
        HttpResponse::Ok().body("Account created successfully")
    } else {
        HttpResponse::InternalServerError().body("Failed to create account")
    }
}

#[put("/update")]
pub async fn update_account(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    "Account update"
}

#[delete("/delete/{id}")]
pub async fn delete_account(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    "Account delete"
}