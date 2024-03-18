use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use surrealdb::{engine::remote::ws::Wss, opt::auth::Root, sql::Thing, Surreal};
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
use once_cell::sync::Lazy;
//  use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;


static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

async fn create_new_district(Json(district): Json<District>) -> String {
    println!("Create new district handler called ");
    DB.connect::<Wss>("localhost:8000").await.unwrap();
    // he mutex to access the database connection

    DB.signin(Root {
        username: "root",
        password: "test12345",
    })
    .await
    .expect("Error: Unable to login to Surreal DB instance with credentials.");

    DB.use_ns("bd")
        .use_db("bd")
        .await
        .expect("Error: Unable to to specified namespace/database");

    let new_entry: Vec<Record> = DB
        .create("table_of_districts_rana")
        .content(district)
        .await
        .expect("Error: Unable to create new entry");

    format!("Added the district: {:?}", new_entry)
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // let db = Arc::new(Mutex::new(
    //     Surreal::new::<Wss>("generalpione.preciqprojects.com")
    //         .await
    //         .expect("Error: Unable to connect to surreal db instance"),
    // ));
    DB.connect::<Wss>("generalpione.preciqprojects.com").await?;
    let app = Router::new().route("/districts", post(create_new_district));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    dbg!("Server listening on port {}", "localhost:3000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
