use axum::{routing::post, Json, Router};
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

async fn create_new_district(
    Json(district): Json<District>,
    db: axum::extract::Extension<Surreal<Wss>>,
) -> String {
    println!("Create new district handler called ");
    db.value()
        .signin(Root {
            username: "rooddt",
            password: "test1sfe34444dfdwewrew5",
        })
        .await
        .expect("Error: Unable to login to Surreal DB instance with credentials.");

    db
        .use_ns("bd")
        .use_db("bd")
        .await
        .expect("Error: Unable to to specified namespace/database");

    let new_entry: Vec<Record> = db
        .value()
        .create("table_of_districts_rana")
        .content(district)
        .await
        .expect("Error: Unable to create new entry");

    format!("Added the district: {:?}", new_entry)
}

async fn create_new_user(Json(_user): Json<User>, db: axum::extract::Extension<Surreal<Wss>>) -> String {
    // Implement your logic for creating a new user here
    unimplemented!()
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Wss>("generalpione.preciqprojects.com")
        .await
        .expect("Error: Unable to connect to surreal db instance");

    let app = Router::new()
        .route("/districts", post(create_new_district))
        .route("/users", post(create_new_user)); // Add additional routes as needed

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    dbg!("Server listening on port {}", "localhost:3000");
    axum::serve(listener, app.into_make_service()).await.unwrap();

    Ok(())
}
