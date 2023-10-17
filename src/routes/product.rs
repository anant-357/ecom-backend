use actix_web::{delete, post, get, web, Responder, HttpResponse, HttpRequest};
use crate::models::Product;

#[get("/{id}")]
async fn get_product(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    println!("{}", id);

    // check for authenticity
    HttpResponse::Ok().json("Valid")
}

#[delete("/{id}")]
async fn delete_product(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    //delete user
    HttpResponse::Ok().json("Successful")
}

#[post("/")]
async fn add_product(user: web::Json<Product>) -> impl Responder {
    // add user
    HttpResponse::Ok().json("Successful")
}

