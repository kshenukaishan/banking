use crate::database::user_query::{get_user_by_id, update_user};
use crate::models::update_user_request::UpdateUserRequest;
use crate::utils::get_user_id;
use crate::AppState;
use actix_web::web::Data;
use actix_web::{get, put, web, HttpRequest, HttpResponse, Responder};

#[get("/user")]
pub async fn get_user(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let db = &state.db.lock().unwrap();
    let user = get_user_by_id(&db, get_user_id(&req)).await;
    HttpResponse::Ok().json(user)
}


#[put("/update-user")]
pub async fn update(
    state: Data<AppState>,
    req: HttpRequest,
    data: web::Json<UpdateUserRequest>) -> impl Responder {
    let db = &state.db.lock().unwrap();
    let user = get_user_by_id(&db, get_user_id(&req)).await;
    update_user(&db, user.unwrap().id, &*data).await;
    let user = get_user_by_id(&db, get_user_id(&req)).await;
    HttpResponse::Ok().json(user)
}

// #[delete("/user/{id}")]
// pub async fn delete_user(user_id: Path<u32>) -> impl Responder {
//     "User Delete: {user_id}"
// }