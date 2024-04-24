#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub published_at: String,
    pub content: String,
}
