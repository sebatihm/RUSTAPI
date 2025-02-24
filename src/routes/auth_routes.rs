use actix_web::web;

use super::handlers;


pub fn config(config : &mut web::ServiceConfig){
    config.service(web::scope("/auth")
        .service(handlers::auth::login)
        .service(handlers::auth::register)

    );
}