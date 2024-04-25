use std::error::Error;

use async_trait::async_trait;
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, IndexParts};
use serde_json::Value;

use crate::domain::gateway::BookManager;
use crate::domain::model;

const INDEX_BOOK: &str = "book_idx";

pub struct ElasticSearchEngine {
    client: Elasticsearch,
}

impl ElasticSearchEngine {
    pub fn new(address: &str) -> Result<Self, Box<dyn Error>> {
        let transport = Transport::single_node(address)?;
        let client = Elasticsearch::new(transport);
        Ok(ElasticSearchEngine { client })
    }
}

#[async_trait]
impl BookManager for ElasticSearchEngine {
    async fn index_book(&self, b: &model::Book) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .index(IndexParts::Index(INDEX_BOOK))
            .body(b)
            .send()
            .await?;
        let response_body = response.json::<Value>().await?;
        Ok(response_body["_id"].as_str().unwrap().into())
    }
}
