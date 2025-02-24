use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse};
use entity::user;
use sea_orm::{EntityTrait, QueryFilter, Set, Condition};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use serde::Deserialize;
use serde::Serialize;
use sha256::digest;

use crate::utils::app_state::{self, AppState};
use crate::utils::jwt::encode_jwt;

#[derive(Serialize, Deserialize)]
struct RegisterModel{
    name: String,
    email: String,
    password: String
}

#[derive(Serialize, Deserialize)]
struct LoginModel{
    email: String,
    password: String
}

#[post("/register")]
pub async fn register(app_state : web::Data<AppState> , register_json: web::Json<RegisterModel>) -> HttpResponse{

    //Creating a new User 
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password : Set(digest(register_json.password.clone())),
        ..Default::default()
    //Inserting the user into the database
    }.insert(&app_state.db).await.unwrap();




    HttpResponse::Ok()
        .status(StatusCode::from_u16(201).unwrap())
        .json(user_model)
}

#[post("/login")]
pub async fn login(app_state : web::Data<AppState> , login_json: web::Json<LoginModel>) -> HttpResponse{

    //Fetching the user info
    let user_model = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq (&login_json.email))
                .add(entity::user::Column::Password.eq (digest(&login_json.password)))
        ).one(&app_state.db).await.unwrap();

    
    //Getting the user data 
    let user_data = user_model.unwrap();

    //Generating jwt
    let token = encode_jwt(user_data.email, user_data.id).unwrap();

    // Returning the token
    HttpResponse::Ok()
        .status(StatusCode::from_u16(200).unwrap())
        .body(format!("{{'token' : '{}'}}", token))

}