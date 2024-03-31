
use surrealdb_with_axum_crud::run;


#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    run().await;
    Ok(())
}