mod book;
mod controllers;

use actix_web::{ web, App, HttpServer};
use mongodb::Client;
use controllers::get::{get_book, get_books};
use controllers::create::create_book;
use controllers::delete::delete_book;
use controllers::update::update_book;

const COLLECTION_NAME: &str = "books";
const DB_NAME: &str = "actix_rust_crud_example";




#[actix_web::main]
async fn main() -> std::io::Result<()>{

    println!("Actix Rust CRUD Example... With MongoDB");

    let client = Client::with_uri_str("mongodb://127.0.0.1:27017/actix_rust_crud_example").await.expect("failed to connect to db");

    HttpServer::new(move || {
        
    App::new().app_data(web::Data::new(client.clone()))
    .service(get_book).service(get_books)
    .service(create_book).service(delete_book).service(update_book)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}