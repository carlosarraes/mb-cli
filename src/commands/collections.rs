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
    #[tabled(rename = "Parent ID")]
    parent_id: String,
    #[tabled(rename = "Personal")]
    personal: bool,
}

pub fn run(client: &MetabaseClient, json: bool) -> Result<()> {
    let collections = client.list_collections()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&collections)?);
        return Ok(());
    }

    let rows: Vec<Row> = collections
        .into_iter()
        .map(|c| Row {
            id: c.id,
            name: c.name,
            parent_id: c.parent_id.map_or("-".into(), |id| id.to_string()),
            personal: c.personal_owner_id.is_some_and(|v| !v.is_null()),
        })
        .collect();
    output::print_table(&rows);
    Ok(())
}
