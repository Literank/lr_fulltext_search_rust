use std::error::Error;

use async_trait::async_trait;

use crate::domain::model;

#[async_trait]
pub trait BookManager: Send + Sync {
    async fn index_book(&self, b: &model::Book) -> Result<String, Box<dyn Error>>;
}
