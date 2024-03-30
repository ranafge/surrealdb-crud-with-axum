use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use surrealdb::{engine::remote::ws::Wss, opt::auth::Root, Surreal};

// #[derive(Debug, Deserialize)]
// struct Record {
//     id: Thing
// }


#[derive(Debug, Deserialize)]
struct Entry {
    
    name: String,
    population: usize,
    climate: String,
    international_border: bool,
    country: String,
    immediate_upper_division: String,
    
}
pub async fn read_entries() -> String   {
    dotenv().ok();

    let db = Surreal::new::<Wss>(env::var("db_url").unwrap())
        .await.expect("Error: unable to connect to database");
    db.signin(Root {
        username: env::var("db_username").unwrap().as_str(),
        password: env::var("db_password").unwrap().as_str(),
    }).await.expect("Error: unable to login to database");

    db.use_ns(env::var("use_ns").unwrap().as_str()).use_db(env::var("use_db").unwrap().as_str()).await.expect("Error: unable to specified namespace/database");

    let entries:Vec<Entry> = db.select("district_rana").await.expect("Unable to read database entries");

    entries.iter().for_each(|entry| {
    println!("{}", entry.name);
    println!("{}", entry.climate);
    println!("{}", entry.country);
    println!("{}", entry.international_border);
    println!("{}", entry.immediate_upper_division);
    println!("{}", entry.population);
  

    
});


    format!("The database entities are: {:?}", &entries)

}