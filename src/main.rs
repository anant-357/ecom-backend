use actix_web::{web, middleware::Logger, HttpServer, App, Responder};
use db::Db;
use env_logger::Env;
mod models;
mod db;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let db: db::Db = Db::with_uri(uri.as_str()).await.unwrap();

    HttpServer::new(move || {
        App::new().wrap(Logger::default())
            .app_data(web::Data::new(db.get_db().clone()))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::scope("/api").service(routes::home))
    }).bind(("127.0.0.1", 8080))?.run().await
}
