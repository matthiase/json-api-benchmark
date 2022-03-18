mod characters;
mod db;
mod error;
mod species;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

pub struct AppData {
    pub db: sqlx::postgres::PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let dsn = env::var("DATABASE_URL").expect("Missing environment variable: DATABASE_URL");

    let pg_pool = db::connect(&dsn).await.unwrap();

    let context = web::Data::new( AppData {
        db: pg_pool
    });

    HttpServer::new(move || {
        App::new()
            .app_data(context.clone()) 
            .route("/characters", web::get().to(characters::index))
            .route("/species", web::get().to(species::index))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
