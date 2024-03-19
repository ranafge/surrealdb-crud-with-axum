use std::env;

use axum::{routing::post, Json, Router};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
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

async fn create_new_district(Json(district): Json<District>) -> String {
    println!("Create new district handler called ");
    let db = Surreal::new::<Wss>("generalpione.preciqprojects.com")
        .await
        .expect("Error: Unable to connect to surreal db instance");
    db.signin(Root {
        username: "root",
        password: "test12345",
    })
    .await
    .expect("Error: Unable to login to Surreal DB instance with credentials.");

    db.use_ns("bd")
        .use_db("bd")
        .await
        .expect("Error: Unable to to specified namespace/database");

    let new_entry: Vec<Record> = db
        .create("table_of_districts_rana")
        .content(district)
        .await
        .expect("Error: Unable to create new entry");

    format!("Added the district: {:?}", new_entry)
}
use dotenv::dotenv;
#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    dotenv().ok();
    let db_url = env::var("db_url").unwrap();
    
    println!("the db urli si {}", db_url);
    // let app = Router::new().route("/districts", post(create_new_district));
    // let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // dbg!("Server listening on port {}", "localhost:3000");
    // axum::serve(listener, app.into_make_service())
    //     .await
    //     .unwrap();

    Ok(())
}
