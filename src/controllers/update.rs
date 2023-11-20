
use crate::{book::BookContent, DB_NAME, COLLECTION_NAME};


// get all books from db

use actix_web::{web, put, HttpResponse};
use mongodb::{Client, Collection, bson::oid::ObjectId, bson::doc};



#[put("/api/books/{_id}")]
async fn update_book(client: web::Data<Client>, _id: web::Path<String>, form: web::Form<BookContent>) -> HttpResponse {

    let _id = _id.into_inner();

    let object_id = match ObjectId::parse_str(&_id) {
        Ok(object_id) => object_id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format specified"),
    };

    let collection: Collection<BookContent>  = client.database(DB_NAME).collection(COLLECTION_NAME);

    match collection.find_one_and_replace(doc! {"_id": object_id}, form.into_inner(), None).await {
        Ok(_) => HttpResponse::Ok().body("Updated book"),
        Err(_err) => HttpResponse::Ok().body(_err.to_string())
    }

}