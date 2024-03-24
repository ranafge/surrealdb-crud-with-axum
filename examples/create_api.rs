use axum::{routing::post, Extension, Json, Router};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize)]
struct District {
    name: String,
    population: usize,
    climate: String,
    international_border: bool,
    country: String,
    immediate_upper_division: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(unused)]
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateDistrictPopulation {
    id: String,
    population: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Population {
    population: usize,
}

type Db = Extension<Arc<Surreal<Client>>>;

async fn create_db_instance() -> Surreal<Client> {
    dotenv().ok();
    let db = Surreal::new::<Wss>(env::var("db_url").unwrap().as_str())
        .await
        .expect("Error: Unable to connect to surrealdb instance");
    db.signin(Root {
        username: env::var("db_username").unwrap().as_str(),
        password: env::var("db_password").unwrap().as_str(),
    })
    .await
    .expect("Error: Unable to login to Surreal DB instance with credentials");

    db.use_ns("bd")
        .use_db("bd")
        .await
        .expect("Error: Unable to connect to specified namespace/database");
    db
}

async fn update_population_of_existing_district(
    Extension(db_instance): Db,
    Json(update_distr_population): Json<UpdateDistrictPopulation>,
) -> String {
    let id = format!("{}", update_distr_population.id);
    let update_distr_population: Option<Record> = db_instance
        .update(("table_districts", id))
        .merge(Population {
            population: update_distr_population.population,
        })
        .await
        .expect("Error: Unable to update population");
    format!(
        "Update population of existing district: {:?}",
        update_distr_population
    )
}

async fn create_new_district(Json(district): Json<District>) -> String {
    dotenv().ok();
    println!("Create new district handler called ");
    let db = Surreal::new::<Wss>(env::var("db_url").unwrap())
        .await
        .expect("Error: Unable to connect to surreal db instance");
    db.signin(Root {
        username: env::var("db_username").unwrap().as_str(),
        password: env::var("db_password").unwrap().as_str(),
    })
    .await
    .expect("Error: Unable to login to Surreal DB instance with credentials.");

    db.use_ns("bd")
        .use_db("bd")
        .await
        .expect("Error: Unable to to specified namespace/database");

    let new_entry: Vec<Record> = db
        .create("table_districts")
        .content(district)
        .await
        .expect("Error: Unable to create new entry");

    format!("Added the district: {:?}", new_entry)
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let unified_db_instance = Arc::new(create_db_instance().await);
    let app = Router::new()
        .route("/districts", post(create_new_district))
        .route(
            "/update-districts",
            post(update_population_of_existing_district),
        )
        .layer(Extension(unified_db_instance));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    dbg!("Server listening on port {}", "localhost:3000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
