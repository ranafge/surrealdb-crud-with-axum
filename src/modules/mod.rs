use std::sync::Arc;

use axum::{routing::{get, post}, Extension, Router};
mod hello_world;
mod db_instance;
mod read_entry;
mod create_entry;


use hello_world::hello;
use read_entry::read_entries;
use db_instance::create_db_instance;
use create_entry::create_new_district;




pub async fn create_routes() -> Router {
    let unified_db_instance = Arc::new(create_db_instance().await);
    Router::new().route("/", get(hello))
    .route("/districts", get(read_entries))
    .route("/create_entry", post(create_new_district))
    .layer(Extension(unified_db_instance  ))



}