use axum::{routing::get ,Json,Router};
use serde_json::json;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::new("info"))
        .init();


    let app  = Router::new()
        .route("/health", get(health));


    //TODO: EXTRACT PORT AND USE FOR ENV
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind port");

    info!("Montra Hospitality ML API Listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start!")

}


async  fn health()-> Json<serde_json::Value>{
    Json(json!({
        "status":"ok",
        "service": "Montra-Hospitality-Intel"
    }))
}
