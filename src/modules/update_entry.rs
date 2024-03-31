use std::sync::Arc;

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

type Db = Extension<Arc<Surreal<Client>>>; 

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePopulation {
    id: String,
    population: usize,
}
#[derive(Debug, Serialize, Deserialize)]
struct Population {
    population: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: Thing,
}



use tracing::{info, debug};
pub async fn update_population(Extension(db_instance): Db, Json(population): Json<UpdatePopulation>) -> String {
    info!("Updating population");
    let id = format!("{}", population.id);
    let update_population_entry:Option<Record> = db_instance.update(("district_rana", id))
        .merge( Population { population: population.population} )
        .await.expect("Error: Unable to update population   entry");
    debug!(" Database entry updated");
    format!("Updated the district population : {:?}", update_population_entry)

}