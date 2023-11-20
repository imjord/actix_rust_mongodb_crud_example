
use crate::{book::Book, DB_NAME, COLLECTION_NAME};


// get all books from db

use actix_web::{web, delete, HttpResponse};
use mongodb::{Client, Collection, bson::oid::ObjectId, bson::doc};



#[delete("/api/books/{_id}")]
async fn delete_book(client: web::Data<Client>, _id: web::Path<String>) -> HttpResponse {

    let _id = _id.into_inner();

    let object_id = match ObjectId::parse_str(&_id) {
        Ok(object_id) => object_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format specified"),
    };

    let collection: Collection<Book>  = client.database(DB_NAME).collection(COLLECTION_NAME);

    match collection.delete_one(doc! {"_id": object_id}, None).await {
        Ok(_) => HttpResponse::Ok().body("Deleted book"),
        Err(_err) => HttpResponse::Ok().body(_err.to_string())
    }

}