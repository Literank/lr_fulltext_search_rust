use std::error::Error;
use std::sync::Arc;

use crate::domain::gateway;
use crate::domain::model;

pub struct BookOperator {
    book_manager: Arc<dyn gateway::BookManager>,
}

impl BookOperator {
    pub fn new(b: Arc<dyn gateway::BookManager>) -> Self {
        BookOperator { book_manager: b }
    }

    pub async fn create_book(&self, b: model::Book) -> Result<String, Box<dyn Error>> {
        Ok(self.book_manager.index_book(&b).await?)
    }

    pub async fn search_books(&self, q: &str) -> Result<Vec<model::Book>, Box<dyn Error>> {
        Ok(self.book_manager.search_books(q).await?)
    }
}
