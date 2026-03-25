use anyhow::Result;
use tabled::Tabled;

use crate::client::MetabaseClient;
use crate::output;

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "DB Type")]
    database_type: String,
    #[tabled(rename = "Base Type")]
    base_type: String,
    #[tabled(rename = "Semantic")]
    semantic_type: String,
}

pub fn run(client: &MetabaseClient, database: &str, table: &str) -> Result<()> {
    let db_id = client.resolve_database(database)?;
    let table_id = client.resolve_table(db_id, table)?;
    let metadata = client.table_query_metadata(table_id)?;
    let rows: Vec<Row> = metadata
        .fields
        .into_iter()
        .map(|f| Row {
            id: f.id,
            name: f.name,
            database_type: f.database_type,
            base_type: f.base_type,
            semantic_type: f.semantic_type.unwrap_or_default(),
        })
        .collect();
    output::print_table(&rows);
    Ok(())
}
