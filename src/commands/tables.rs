use anyhow::Result;
use tabled::Tabled;

use crate::client::MetabaseClient;
use crate::output;

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Schema")]
    schema: String,
    #[tabled(rename = "Name")]
    name: String,
}

pub fn run(client: &MetabaseClient, database: &str) -> Result<()> {
    let db_id = client.resolve_database(database)?;
    let metadata = client.database_metadata(db_id)?;
    let rows: Vec<Row> = metadata.tables.into_iter()
        .map(|t| Row {
            id: t.id,
            schema: t.schema.unwrap_or_default(),
            name: t.name,
        })
        .collect();
    output::print_table(&rows);
    Ok(())
}
