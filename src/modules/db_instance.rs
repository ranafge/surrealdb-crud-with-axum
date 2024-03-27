use dotenv::dotenv;
use std::env;
use surrealdb::{engine::remote::ws::{Client, Wss}, opt::auth::Root, Surreal};


pub async fn create_db_instance() -> Surreal<Client> {
    dotenv().ok();
    let db = Surreal::new::<Wss>(env::var("db_url").unwrap())
        .await.expect("Error: Unable to connect to surreal database instance");
    db.signin(Root {
        username: env::var("db_username").unwrap().as_str(),
        password: env::var("db_password").unwrap().as_str(),
    }).await.expect("Error: Unable to login to database instance");

    db.use_ns("bd")
       .use_db("bd")
       .await.expect("Error: Unable to login to database instance");
    db.use_ns(env::var("use_ns").unwrap())
       .use_db(env::var("use_db").unwrap())
       .await.expect("Error: Unable to connect to specified namespace/dt database instance");

    db 

}