use std::sync::Arc;

use axum::{response::Html, Extension, Form};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

#[derive(Debug, Deserialize)]
pub struct Input {
   pub name: String,
   pub email:String
}
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
    name: String,
  

}
pub async fn accept_from(Extension(db_instance): Db,Form(district): Form<District>)-> Html<&'static str> {
    
    tracing::info!("You are going to create a new district form user form");
    let new_entry: Vec<Record> = db_instance

        .create("district_rana")
        .content(district)
        .await
        .expect("Error: Unable to create district_rana table");
    tracing::debug!(" Creating database entry complete {:?}", new_entry);
 
    Html(
        r#"
        <html>
            <head>
                <title>SurrealDB</title>
            </head>
            <body>
            
        <h2>District Form</h2>
        <h2>Submitted successfully</h2>
        </body>
        </html>
        "#)
}
pub async fn show_from() -> Html<&'static str> {
    tracing::info!("You are opened Show_from endpoint");
    Html(
        r#"
        <html>
            <head>
                <title>SurrealDB</title>
            </head>
            <body>
            <h2>District Form</h2>
            <form action="/show_form" method="post">
            <label for="name">Name:</label><br>
            <input type="text" id="name" name="name"><br>
            
            <label for="population">Population:</label><br>
            <input type="number" id="population" name="population" required><br>
            
            <label for="climate">Climate:</label><br>
            <input type="text" id="climate" name="climate"><br>
            
            <label>International Border:</label><br>
            <input type="radio" id="international_border_true" name="international_border" value="true">
            <label for="international_border_true">True</label>
            
            <input type="radio" id="international_border_false" name="international_border" value="false">
            <label for="international_border_false">False</label><br>
            
            <label for="country">Country:</label><br>
            <input type="text" id="country" name="country"><br>
            
            <label for="immediate_upper_division">Immediate Upper Division:</label><br>
            <input type="text" id="immediate_upper_division" name="immediate_upper_division"><br><br>
            
            <input type="submit" value="Submit">
        </form>
     
        </body>
        </html>
        "#
    )
    
}