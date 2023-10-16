use actix_web::{delete, post, get, web, Responder, HttpResponse, HttpRequest};
use crate::models::User;

enum Validity {
    Valid,
    Invalid
}

#[get("/auth/check/{name}/{pass}")]
async fn home(path: web::Path<(String, String)>) -> impl Responder {
    let (name, pass) = path.into_inner();
    // check for authenticity
    HttpResponse::Ok().json("Valid")
}

#[delete("/auth/delete/{name}")]
async fn delete_user(path: web::Path<(String)>) -> impl Responder {
    let name = path.into_inner();
    //delete user
    HttpResponse::Ok().json("Successful")
}

#[post("/auth/add")]
async fn home_post(info: web::Json<User>) -> impl Responder {
    // add user
    HttpResponse::Ok().json("Successful")
}

