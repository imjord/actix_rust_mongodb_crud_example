use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Book {
    pub author: String,
    pub title: String,
    pub pages: u32,
}


impl Book {
    pub fn new(author: String, title: String, pages: u32) -> Self {
        Self {
            author,
            title,
            pages
        }
    }
}