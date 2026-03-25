use anyhow::Result;
use tabled::Tabled;

use crate::client::{Card, MetabaseClient};
use crate::output;

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Query Type")]
    query_type: String,
    #[tabled(rename = "DB ID")]
    database_id: String,
    #[tabled(rename = "Collection ID")]
    collection_id: String,
    #[tabled(rename = "Archived")]
    archived: bool,
    #[tabled(rename = "Updated At")]
    updated_at: String,
}

fn to_row(c: Card) -> Row {
    Row {
        id: c.id,
        name: c.name,
        query_type: c.query_type.unwrap_or_else(|| "-".into()),
        database_id: c.database_id.map_or("-".into(), |id| id.to_string()),
        collection_id: c.collection_id.map_or("-".into(), |id| id.to_string()),
        archived: c.archived,
        updated_at: c.updated_at,
    }
}

pub fn run(
    client: &MetabaseClient,
    collection: Option<&str>,
    search: Option<&str>,
    archived: bool,
    json: bool,
) -> Result<()> {
    let cards = client.list_cards()?;

    let collection_id = collection
        .map(|c| client.resolve_collection(c))
        .transpose()?;

    let search_lower = search.map(|s| s.to_lowercase());

    let filtered: Vec<Card> = cards
        .into_iter()
        .filter(|c| archived || !c.archived)
        .filter(|c| match collection_id {
            Some(id) => c.collection_id == Some(id),
            None => true,
        })
        .filter(|c| match &search_lower {
            Some(text) => c.name.to_lowercase().contains(text),
            None => true,
        })
        .collect();

    if json {
        println!("{}", serde_json::to_string_pretty(&filtered)?);
        return Ok(());
    }

    let rows: Vec<Row> = filtered.into_iter().map(to_row).collect();
    output::print_table(&rows);
    Ok(())
}
