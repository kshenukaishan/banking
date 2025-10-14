use crate::database::account_query;
use crate::models::account::Account;
use crate::models::update_account_request::UpdateAccountRequest;
use crate::{database, utils, AppState};
use actix_web::web::{Data, Path};
use actix_web::{delete, post, put, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[post("/new")]
pub async fn create(state: web::Data<AppState>, account: web::Json<Account>) -> impl Responder {
    let db = state.db.lock().unwrap();

    let success = database::account_query::create_account(&db, account).await;

    if success {
        HttpResponse::Ok().body("Account created successfully")
    } else {
        HttpResponse::InternalServerError().body("Failed to create account")
    }
}

#[put("/show/{id}")]
pub async fn show(req: HttpRequest, state: web::Data<AppState>, id: Path<u64>) -> impl Responder {
    let db = state.db.lock().unwrap();
    let user_id = utils::get_user_id(&req);
    let account = account_query::get_account(&db, id.into_inner()).await;

    if account.user_id != user_id {
        HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Account does not exist"
        }))
    } else {
        HttpResponse::Ok().json(account)
    }
}

#[put("/update/{id}")]
pub async fn update(
    req: HttpRequest,
    state: web::Data<AppState>,
    data: Data<UpdateAccountRequest>,
    id: Path<u64>,
) -> impl Responder {
    let db = state.db.lock().unwrap();
    let user_id = utils::get_user_id(&req);
    let account = account_query::get_account(&db, id.into_inner()).await;

    if account.user_id != user_id {
        HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorised user ID"
        }));
    }

    account_query::update(&db, &data, account.id).await;
    let account = account_query::get_account(&db, account.id).await;
    HttpResponse::Ok().json(account)
}

#[delete("/delete/{id}")]
pub async fn delete(req: HttpRequest, state: web::Data<AppState>, id: Path<u64>) -> impl Responder {
    let db = state.db.lock().unwrap();
    let user_id = utils::get_user_id(&req);
    let account = account_query::get_account(&db, id.into_inner()).await;

    if account.user_id != user_id {
        HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Account does not exist"
        }));
    }

    account_query::delete(&db, account.id).await;
    HttpResponse::Ok().json(json!({
        "status": "ok",
    }))
}
