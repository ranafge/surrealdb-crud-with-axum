use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::net::TcpListener;

use std::sync::Arc;

use axum::{
    extract::{Path,  Query},
    routing::{delete, post},
    Extension, Json, Router,
};
use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

// All structs   here
#[derive(Debug, Serialize, Deserialize)]
struct District {
    id: String,
    population: usize,
    climate: String,
    international_border: String,
    country: String,
    immediate_upper_division: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
struct Population {
    population: usize,
}
#[derive(Debug, Serialize, Deserialize)]
struct UpdateDistrictPop {
    id: String,
    population: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct DistrictId {
    id: String,
}
// end All structs here

// Database instance Alias type

type Db = Extension<Arc<Surreal<Client>>>;

// Functions handlers

async fn create_database_instance() -> Surreal<Client> {
    dotenv().ok();
    let db = Surreal::new::<Wss>(env::var("db_url").unwrap())
        .await
        .expect("Error: Unable to connect to surreal database instance");

    db.signin(Root {
        username: env::var("db_username").unwrap().as_str(),
        password: env::var("db_password").unwrap().as_str(),
    })
    .await
    .expect("Error: Unable to login to Surreal database instance");
    db.use_ns(env::var("use_ns").unwrap())
        .use_db(env::var("use_db").unwrap())
        .await
        .expect("Error: Unable to connect to specified namespace/dt database instance");

    db
}

async fn create_new_district(Extension(db_instance): Db, Json(district): Json<District>) -> String {
    let new_entry: Vec<Record> = db_instance
        .create("district_rana")
        .content(district)
        .await
        .expect("Error: Unable to create district_rana table");

    format!("Added the district to the database : {:?}", new_entry)
}

async fn update_pop_of_existing_district(
    Extension(db_instance): Db,
    Json(update_pop): Json<UpdateDistrictPop>,
) -> String {
    let id = format!("{}", update_pop.id);
    let updated_entry: Option<Record> = db_instance
        .update(("district_rana", id))
        .merge(Population {
            population: update_pop.population,
        })
        .await
        .expect("Error: Unable to update district_rana table population");
    format!("Updated the district population : {:?}", updated_entry)
}

async fn delete_district(
    Extension(db_instance): Db, Path(table_name): Path<String>
     ,Query(district_id): Query<DistrictId>) -> String 
     
     {
    let deleted_entry: Option<Record> = db_instance.delete(
        (table_name,district_id.id  )
    ).await.expect("Unable to delete");
    format!("Deleting the district from the database: {:?}", deleted_entry)
}



// End for function handler

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let unified_db_instance = Arc::new(create_database_instance().await);
    let app = Router::new()
        .route("/districts", post(create_new_district))
        .route("/update_districts", post(update_pop_of_existing_district))
        .route("/districts/:id", delete(delete_district))
        .layer(Extension(unified_db_instance));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    dbg!("Server listening on port 3000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
