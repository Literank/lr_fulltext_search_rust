use std::error::Error;

use async_trait::async_trait;
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, IndexParts, SearchParts};
use serde_json::{json, Value};

use crate::domain::gateway::BookManager;
use crate::domain::model;

const INDEX_BOOK: &str = "book_idx";

pub struct ElasticSearchEngine {
    client: Elasticsearch,
    page_size: u32,
}

impl ElasticSearchEngine {
    pub fn new(address: &str, page_size: u32) -> Result<Self, Box<dyn Error>> {
        let transport = Transport::single_node(address)?;
        let client = Elasticsearch::new(transport);
        Ok(ElasticSearchEngine { client, page_size })
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

    async fn search_books(&self, q: &str) -> Result<Vec<model::Book>, Box<dyn Error>> {
        let response = self
            .client
            .search(SearchParts::Index(&[INDEX_BOOK]))
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
        for hit in response_body["hits"]["hits"].as_array().unwrap() {
            let source = hit["_source"].clone();
            let book: model::Book = serde_json::from_value(source).unwrap();
            books.push(book);
        }
        Ok(books)
    }
}
