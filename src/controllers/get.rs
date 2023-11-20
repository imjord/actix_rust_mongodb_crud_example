
use crate::{book::Book, DB_NAME, COLLECTION_NAME};


// get all books from db

use actix_web::{web, get, HttpResponse};
use mongodb::{Client, Collection, bson::oid::ObjectId, bson::doc};
use futures_util::stream::StreamExt;


#[get("/api/books")]
async fn get_books(client: web::Data<Client>) -> HttpResponse {

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



#[get("/api/books/{_id}")]
async fn get_book(client: web::Data<Client>, _id: web::Path<String>) -> HttpResponse {

    let _id = _id.into_inner();

    let object_id = match ObjectId::parse_str(&_id) {
        Ok(object_id) => object_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format specified"),
    };

    let collection: Collection<Book>  = client.database(DB_NAME).collection(COLLECTION_NAME);

    match collection.find_one(doc! {"_id": object_id}, None).await {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_err) => HttpResponse::Ok().body(_err.to_string())
    }

}