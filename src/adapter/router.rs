use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use serde_json::json;

use crate::application;
use crate::application::executor;
use crate::domain::model;

pub struct RestHandler {
    book_operator: executor::BookOperator,
}

async fn create_book(
    State(rest_handler): State<Arc<RestHandler>>,
    Json(book): Json<model::Book>,
) -> Result<Json<serde_json::Value>, impl IntoResponse> {
    match rest_handler.book_operator.create_book(book).await {
        Ok(book_id) => Ok(Json(json!({"id": book_id}))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn welcome() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok"
    }))
}

pub fn make_router(wire_helper: &application::WireHelper) -> Router {
    let rest_handler = Arc::new(RestHandler {
        book_operator: executor::BookOperator::new(wire_helper.book_manager()),
    });
    Router::new()
        .route("/", get(welcome))
        .route("/books", post(create_book))
        .with_state(rest_handler)
}
