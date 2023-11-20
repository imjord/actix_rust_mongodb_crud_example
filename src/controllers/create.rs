use crate::{book::BookContent, DB_NAME, COLLECTION_NAME};


// get all books from db

use actix_web::{web, post, HttpResponse};
use mongodb::{Client, Collection};


#[post("/api/books")]
async fn create_book(client: web::Data<Client>, form: web::Form<BookContent>) -> HttpResponse {

    let collection: Collection<BookContent>  = client.database(DB_NAME).collection(COLLECTION_NAME);


    let result = collection.insert_one(form.into_inner(), None).await;

    match  result {
        Ok(_) => HttpResponse::Ok().body("Book added to collection"),
        Err(_err) => HttpResponse::Ok().body(_err.to_string())
    }
    


}