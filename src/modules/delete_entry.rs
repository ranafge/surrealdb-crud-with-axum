use std::sync::Arc;

use axum::{Extension, extract::Path, extract::Query};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

type Db = Extension<Arc<Surreal<Client>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DistrictId {
    id: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: Thing,
}

pub async fn delete_district(
    Extension(db_instance): Db,
    Path(table_name): Path<String>,
    Query(district_id): Query<DistrictId>,
) -> String {
    let deleted_entry: Option<Record> = db_instance
        .delete((table_name, district_id.id))
        .await
        .expect("Unable to delete");
    format!(
        "Deleting the district from the database: {:?}",
        deleted_entry
    )
}
