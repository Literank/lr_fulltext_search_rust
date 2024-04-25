use std::error::Error;

use async_trait::async_trait;

use crate::domain::model;

#[async_trait]
pub trait BookManager: Send + Sync {
    async fn index_book(&self, b: &model::Book) -> Result<String, Box<dyn Error>>;
    async fn search_books(&self, q: &str) -> Result<Vec<model::Book>, Box<dyn Error>>;
}
