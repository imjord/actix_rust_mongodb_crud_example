
use crate::{book::Book, DB_NAME, COLLECTION_NAME};


// get all books from db

use actix_web::{web, get, HttpResponse};
use mongodb::{Client, Collection};
use futures_util::stream::StreamExt;


#[get("/api/book")]
async fn get_users(client: web::Data<Client>) -> HttpResponse {

    let collection: Collection<Book>  = client.database(DB_NAME).collection(COLLECTION_NAME);

    let mut cursor = collection.find(None, None).await.expect("Cant fetch books");

    let mut books: Vec<Book> = Vec::new();

    while let Some(doc) = cursor.next().await{
        match doc {
            Ok(book) => {
                books.push(book);
            }
            Err(_err) => {
                return HttpResponse::Ok().body(_err.to_string());
            }
        }
    }
    

    if !books.is_empty() {
        HttpResponse::Ok().json(books)
    } else {
        HttpResponse::NotFound().body("No books found")
    }


}