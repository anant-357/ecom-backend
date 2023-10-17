use actix_web::{delete, post, get, web, Responder, HttpResponse, Scope};
use mongodb::{Client, bson::doc};
use crate::models::User;

const DB_NAME: &str = "ecommerce";
const COLL_NAME: &str = "users";

#[get("/check/{name}/{pass}")]
async fn check(client: web::Data<Client>, path: web::Path<(String, String)>) -> impl Responder {
    let (name, pass) = path.into_inner();
    match client.database(DB_NAME).collection::<User>(COLL_NAME).find_one(doc! {"name": name, "pass": pass}, None).await {
        Ok(Some(_user)) => HttpResponse::Ok().json("Valid"),
        Ok(None) => HttpResponse::Ok().json("Invalid"),
        Err(_) => HttpResponse::InternalServerError().json("server error")        
    }
}

#[get("/{name}")]
async fn get_user(client: web::Data<Client>, path: web::Path<String>)-> impl Responder {
    match client.database(DB_NAME).collection::<User>(COLL_NAME).find_one(doc! {"name": path.into_inner()}, None).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::Ok().json("No such User"),
        Err(_) => HttpResponse::InternalServerError().json("server error")
    }
}

#[delete("/{name}")]
async fn delete_user(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    match client.database(DB_NAME).collection::<User>(COLL_NAME).delete_one(doc! {"name": path.into_inner()}, None).await {
        Ok(res) => {
            match res.deleted_count { 
            1 => HttpResponse::Ok().json("User deleted"),
            _ => HttpResponse::InternalServerError().json("Unable to delete user")
            }
        },
        _ => HttpResponse::InternalServerError().json("Unable to delete user")

    }
}

#[post("/")]
async fn add_user(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    match client.database(DB_NAME).collection(COLL_NAME).insert_one(user, None).await {
        Ok(_) => HttpResponse::Ok().json("User Added"),
        Err(_) => HttpResponse::InternalServerError().json("Unable to add user")
    }
}

pub fn auth()-> Scope{
    return web::scope("/auth")
        .service(check)
        .service(get_user)
        .service(delete_user)
        .service(add_user)
}
