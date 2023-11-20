use serde::{Deserialize, Serialize};
use mongodb::bson::doc;


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Book {
    pub _id: mongodb::bson::oid::ObjectId,
    pub author: String,
    pub title: String,
    pub pages: u32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]

pub struct BookContent {
    pub author: String,
    pub title: String,
    pub pages: u32,
}