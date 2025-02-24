use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};

use crate::utils::app_state::{AppState};


#[get("/test")]
pub async fn user (app_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok()
        .status(StatusCode::from_u16(200).unwrap())
        .body("User verified")
}

#[get("/test1")]
pub async fn hola() -> impl Responder{
    format!("AAAAAAAAAAAAA")
}