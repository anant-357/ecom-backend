use actix_web::{delete, post, get, web, Responder, HttpResponse};
use mongodb::{Client, bson::doc};
use crate::models::Product;

const DB_NAME: &str = "ecommerce";
const COLL_NAME: &str = "products";

#[get("/{id}")]
async fn get_product(client: web::Data<Client>,path: web::Path<u32>) -> impl Responder {
    match client.database(DB_NAME).collection::<Product>(COLL_NAME).find_one(doc! {"id": path.into_inner()}, None).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::Ok().json("No such product"),
        Err(_) => HttpResponse::InternalServerError().json("server error")
    }
}

#[delete("/{id}")]
async fn delete_product(client: web::Data<Client>, path : web::Path<u32>) -> impl Responder {
    match client.database(DB_NAME).collection::<Product>(COLL_NAME).delete_one(doc! {"id": path.into_inner()}, None).await {
      Ok(res) => {
            match res.deleted_count { 
            1 => HttpResponse::Ok().json("Product deleted"),
            _ => HttpResponse::InternalServerError().json("Unable to delete product")
            }
        },
        _ => HttpResponse::InternalServerError().json("Unable to delete product")
    }
}

#[post("/")]
async fn add_product(client: web::Data<Client>, product: web::Json<Product>) -> impl Responder {
    match client.database(DB_NAME).collection::<Product>(COLL_NAME).insert_one(product.into_inner(), None).await {
        Ok(_) => HttpResponse::Ok().json("Product Added"),
        Err(_) => HttpResponse::InternalServerError().json("Unable to add product")
    }
}

