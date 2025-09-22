use crate::models::user_model::User;
use actix_web::{HttpMessage, HttpRequest};
use sqlx::MySqlPool;
use crate::{database};

pub fn get_user_id(req: &HttpRequest) -> u64 {
    let ext = req.extensions();
    ext.get::<u64>().unwrap().to_owned()
}

pub async fn get_authenticated_user(req: &HttpRequest, db: &MySqlPool) -> User {
    database::user_query::get_user_by_id(db, get_user_id(req)).await.unwrap()
}