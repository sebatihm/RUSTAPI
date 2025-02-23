use actix_web::{get, web, Responder};
use sea_orm::{Statement, ConnectionTrait};

use crate::utils::app_state::AppState;



#[get("/index")]
pub async fn get(app_state: web::Data<AppState>) -> impl Responder{
    let query = app_state.db.query_all(Statement::from_string(sea_orm::DatabaseBackend::MySql, "Select * from user;")).await.unwrap();
    format!("AAAAAAAAAAAAA")
}