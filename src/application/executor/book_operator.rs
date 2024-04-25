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
}
