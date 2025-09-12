use actix_web::{delete, get, post, put, Responder};
use actix_web::web::Path;

// #[post("/users")]
// pub async fn create_user() -> impl Responder {
//     "User create"
// }

#[get("/users")]
pub async fn users_list() -> impl Responder {
    "Users List"
}

#[get("/user/{id}")]
pub async fn get_users(user_id: Path<u32>) -> impl Responder {
    "User No: {user_id}"
}

#[put("/user/{id}")]
pub async fn update_user(user_id: Path<u32>) -> impl Responder {
    "User Update: {user_id}"
}

#[delete("/user/{id}")]
pub async fn delete_user(user_id: Path<u32>) -> impl Responder {
    "User Delete: {user_id}"
}