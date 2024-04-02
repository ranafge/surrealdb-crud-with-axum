use std::sync::Arc;

use axum::{routing::{delete, get, post}, Extension, Router};
mod hello_world;
mod db_instance;
mod read_entry;
mod create_entry;
mod  update_entry;
mod delete_entry;
mod my_middleware;
mod tracing_middleware;
mod form;


use hello_world::hello;
use read_entry::read_entries;
use db_instance::create_db_instance;
use create_entry::create_new_district;
use update_entry::update_population;
use delete_entry::delete_district;

use form::show_from;

use self::form::accept_from;






pub async fn create_routes() -> Router {

    let unified_db_instance = Arc::new(create_db_instance().await);
    Router::new().route("/", get(hello))
    .route("/districts", get(read_entries))
    .route("/create_entry", post(create_new_district))
    .route("/update_population", post(update_population))
    .route("/district/:id", delete(delete_district))
    .route("/show_form", get(show_from).post(accept_from))
    .layer(Extension(unified_db_instance  ))
    // .layer(middleware::from_fn(my_middleware))
    .layer(tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init())
    



}