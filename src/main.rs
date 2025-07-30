use axum::{
    routing::get,
    Router,
};
use axum::extract::Path;

async fn show_video(Path(link): Path<String>) -> String {
    return link
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/{link}", get(show_video));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}