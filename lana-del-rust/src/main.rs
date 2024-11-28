use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port");

    axum::serve(listener, app).await.expect("Failed to start server");
}

async fn root() -> &'static str {
    "Pepsi Cola!\n"
}