use axum::{routing::get, Router};
use tcmalloc::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
