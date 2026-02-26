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
    #[tabled(rename = "Engine")]
    engine: String,
}

pub fn run(client: &MetabaseClient) -> Result<()> {
    let databases = client.list_databases()?;
    let rows: Vec<Row> = databases.into_iter()
        .map(|db| Row { id: db.id, name: db.name, engine: db.engine })
        .collect();
    output::print_table(&rows);
    Ok(())
}
