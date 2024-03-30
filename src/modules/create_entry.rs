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



pub async fn create_new_district(Extension(db_instance): Db, Json(district): Json<District>) -> String {
    let new_entry: Vec<Record> = db_instance
        .create("district_rana")
        .content(district)
        .await
        .expect("Error: Unable to create district_rana table");

    format!("Added the district to the database : {:?}", new_entry)
}
