use std::sync::Arc;

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};


type Db = Extension<Arc<Surreal<Client>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    name: String,
    population: usize,
    climate: String,
    international_border: bool,
    country: String,
    immediate_upper_division: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {

    id: Thing,
  

}

use tracing::{info, debug};

pub async fn create_new_district(Extension(db_instance): Db, Json(district): Json<District>) -> String {
    info!("You are going to create a new district");
    let new_entry: Vec<Record> = db_instance

        .create("district_rana")
        .content(district)
        .await
        .expect("Error: Unable to create district_rana table");
    debug!(" Creating database entry complete");
    format!("Added the district to the database : {:?}", new_entry)
}
