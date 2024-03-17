use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::{engine::remote::ws::Wss, opt::auth::Root, Surreal};

#[derive(Debug, Deserialize, Serialize)]
struct District {
    name: String,
    number_of_thana: usize,
    thana: Vec<String>,
    climate: String,
    population: usize,
    international_boder: bool,
    timestamp: DateTime<Utc>,
}


#[derive(Debug, Deserialize)]
struct Record {
    id: Thing,
    name: String,
    number_of_thana: usize,
    thana: Vec<String>,
    #[allow(unused)]
    climate: String,
    population: usize,
    #[allow(unused)]
    international_boder: bool,
    timestamp: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Wss>("generalpione.preciqprojects.com").await?;

    db.signin(Root {
        username: "root",
        password: "test12345",
    })
    .await?;


    db.use_ns("bd").use_db("bd").await?;
    // Delete the table District
    // let delete_table: Vec<Record> = db.delete("district").await?;
    // dbg!(delete_table);

    let rangpur = District {
        name: "Rangpur".to_string(),
        number_of_thana: 1,
        thana: vec!["Mithapukur".to_string()],
        climate: "Rainny".to_string(),
        population: 10_000_00,
        international_boder: false,
        timestamp: Utc::now(),
    };

    let dinajpur = District {
        name: "Dinajpur".to_string(),
        number_of_thana: 1,
        thana: vec!["Fulbari".to_string()],
        climate: "Wet".to_string(),
        population: 20_000_00,
        international_boder: false,
        timestamp: Utc::now(),
    };

    let kurigram = District {
        name: "Kurigram".to_string(),
        number_of_thana: 1,
        thana: vec!["Bhurungamari".to_string()],
        climate: "Sunny".to_string(),
        population: 30_000_00,
        international_boder: true,
        timestamp: Utc::now(),
    };
    // Create a new district with the given name

    // let rangpur_dist: Vec<Record> = db.create("district").content(&rangpur).await?;
    // dbg!(rangpur_dist);
    // let dinajpur_dist: Vec<Record> = db.create("district").content(&dinajpur).await?;
    // dbg!(dinajpur_dist);
    // let kurigram_dist: Vec<Record> = db.create("district").content(&kurigram).await?;
    // dbg!(kurigram_dist);


    // Update the rangpur climate rainy to Dry
    // let update_rangpur_climate: Option<Record> = db
    //     .update(("district", "3pwsqev7kq3gerr6lzpu"))
    //     .merge(District {
    //         climate: "Dry".to_string(),
    //         ..rangpur
    //     })
    //     .await?;

    // dbg!(update_rangpur_climate);

    // we can update the struct field with the struct like that
    // #[derive(Debug, Serialize, Deserialize)]
    // struct ModifyNumberOfThana {
    //     number_of_thana: usize,
    // }
    // let update_number_of_thana_of_rangpur: Option<Record> = db
    //     .update(("district", "3pwsqev7kq3gerr6lzpu"))
    //     .merge(ModifyNumberOfThana { number_of_thana: 2 })
    //     .await?;

    // dbg!(update_number_of_thana_of_rangpur);

    // Update an vec field of the rangpur district using sql query
    // let query = r#"
    // UPDATE district
    // SET thana += ['pirgacha', 'pirgong']
    // WHERE  id= "district:3pwsqev7kq3gerr6lzpu"
    // "#;
    // db.query(query).await?;

    //  update number_of_thana  according to the vector lenght of thana ()
    // let query = r#"
    //  UPDATE district
    //  SET number_of_thana = array::len(thana)

    
    // "#;
    // db.query(query).await?;

      // delete column value
    // let query = r#"
    //     UPDATE  district
    //        SET population = REMOVE 
    //     WHERE id="district:72gnrblsg0m53pxedah4"
    // "#;
    // db.query(query).await?;

    // Print every entry of the struct that select from database 
    // to extract the every filed of struct you have to need Record struct above.
    // let entries: Vec<Record> = db.select("district").await?;
    // entries.iter().for_each(|entry| {
    //     println!("ID: {}", entry.id);   
    //     println!("Total thana: {}", entry.number_of_thana);
    //     println!("Name: {}", entry.name);
    //     println!("Total population: {}", entry.population);
    //     println!("Thana: {:?}", entry.thana);
    //     println!("Created time: {}", entry.timestamp);

       
    // });

    // delete COLUMN
    // let query = r#"
    //     UPDATE  District
    //        SET population = REMOVE 
    //     WHERE id="District:x3we856001c2ibzweexz"
    // "#;
    // db.query(query).await?;

    

    // Delete single record
    // let single_entry_delete : Option<Record> = db.delete(("district", "3pwsqev7kq3gerr6lzpu")).await?;
    // dbg!(single_entry_delete);

    
    // Delete all records in a table
    // db.delete("district").await?;


    Ok(())
}
