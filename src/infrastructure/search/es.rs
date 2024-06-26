use std::error::Error;

use async_trait::async_trait;
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, IndexParts, SearchParts};
use serde_json::{json, Value};

use crate::domain::gateway::BookManager;
use crate::domain::model;

pub struct ElasticSearchEngine {
    client: Elasticsearch,
    index: String,
    page_size: u32,
}

impl ElasticSearchEngine {
    pub fn new(address: &str, index: &str, page_size: u32) -> Result<Self, Box<dyn Error>> {
        let transport = Transport::single_node(address)?;
        let client = Elasticsearch::new(transport);
        Ok(ElasticSearchEngine {
            client,
            index: index.to_string(),
            page_size,
        })
    }
}

#[async_trait]
impl BookManager for ElasticSearchEngine {
    async fn index_book(&self, b: &model::Book) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .index(IndexParts::Index(&self.index))
            .body(b)
            .send()
            .await?;
        let response_body = response.json::<Value>().await?;
        Ok(response_body["_id"].as_str().unwrap().into())
    }

    async fn search_books(&self, q: &str) -> Result<Vec<model::Book>, Box<dyn Error>> {
        let response = self
            .client
            .search(SearchParts::Index(&[&self.index]))
            .from(0)
            .size(self.page_size as i64)
            .body(json!({
                "query": {
                    "multi_match": {
                        "query": q,
                        "fields": vec!["title", "author", "content"],
                    }
                }
            }))
            .send()
            .await?;
        let response_body = response.json::<Value>().await?;
        let mut books: Vec<model::Book> = vec![];
        if let Some(hits) = response_body["hits"]["hits"].as_array() {
            for hit in hits {
                let source = hit["_source"].clone();
                let book: model::Book = serde_json::from_value(source).unwrap();
                books.push(book);
            }
        }
        Ok(books)
    }
}
