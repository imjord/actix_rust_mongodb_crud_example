mod book;
mod controllers;

use book::Book;
use actix_web::{get, post, http, web, App, HttpResponse, HttpServer};
use actix_web::http::header;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use controllers::get::get_users;

const COLLECTION_NAME: &str = "books";
const DB_NAME: &str = "actix_rust_crud_example";




#[actix_web::main]
async fn main() -> std::io::Result<()>{

    println!("Actix Rust CRUD Example... With MongoDB");

    let client = Client::with_uri_str("mongodb://127.0.0.1:27017/actix_rust_crud_example").await.expect("failed to connect to db");

    HttpServer::new(move || {
        
           App::new().app_data(web::Data::new(client.clone())).service(get_users)
   
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}