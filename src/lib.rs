mod modules;


use modules::create_routes;
use tokio::net::TcpListener;

pub async fn run() {
    let app = create_routes().await;
    format!("Server listening on port {}", "3000");
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}