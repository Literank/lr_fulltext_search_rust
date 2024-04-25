mod adapter;
mod application;
mod domain;
mod infrastructure;

const CONFIG_FILE: &str = "config.toml";

#[tokio::main]
async fn main() {
    let c = infrastructure::parse_config(CONFIG_FILE);
    let wire_helper = application::WireHelper::new(&c).expect("Failed to create WireHelper");
    let app = adapter::make_router(&wire_helper);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
