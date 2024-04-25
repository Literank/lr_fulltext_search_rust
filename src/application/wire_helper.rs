use std::sync::Arc;

use crate::domain::gateway;
use crate::infrastructure::search;
use crate::infrastructure::Config;

pub struct WireHelper {
    engine: Arc<search::ElasticSearchEngine>,
}

impl WireHelper {
    pub fn new(c: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let engine = Arc::new(search::ElasticSearchEngine::new(
            &c.search.address,
            &c.search.index,
            c.app.page_size,
        )?);
        Ok(WireHelper { engine })
    }

    pub fn book_manager(&self) -> Arc<dyn gateway::BookManager> {
        Arc::clone(&self.engine) as Arc<dyn gateway::BookManager>
    }
}
