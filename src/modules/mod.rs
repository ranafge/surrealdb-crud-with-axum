use std::sync::Arc;

use axum::{routing::get, Extension, Router};
mod hello_world;
mod db_instance;
mod read_entry;

use hello_world::hello;
use read_entry::read_entries;
use db_instance::create_db_instance;

pub async fn create_routes() -> Router {
    let unified_db_instance = Arc::new(create_db_instance().await);
    Router::new().route("/", get(hello))
    .route("/districts", get(read_entries))
    .layer(Extension(unified_db_instance  ))



}