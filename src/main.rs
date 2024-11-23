use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes = Router::new().route(
        "/hello",
        get(|| async { Html("<strong>Hello World!</strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Listening on: {:?}", addr);

    axum::serve(listener, routes).await.unwrap();
}
