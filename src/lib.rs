mod modules;


use modules::create_routes;
use tokio::net::TcpListener;

pub async fn run() {
    let app = create_routes().await;
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    dbg!("Server listening on port {}", "localhost:3000");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}