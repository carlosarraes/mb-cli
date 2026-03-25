use anyhow::Result;

use crate::client::MetabaseClient;

pub fn run(
    client: &MetabaseClient,
    id_or_name: &str,
    inspect: bool,
    sql: bool,
    json: bool,
) -> Result<()> {
    let card = client.resolve_question(id_or_name)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&card)?);
        return Ok(());
    }

    if inspect {
        println!("{}", serde_json::to_string_pretty(&card.dataset_query)?);
        return Ok(());
    }

    if sql {
        let query_type = card.query_type.as_deref().unwrap_or("");
        if query_type == "native" {
            if let Some(q) = card
                .dataset_query
                .get("native")
                .and_then(|n| n.get("query"))
                .and_then(|q| q.as_str())
            {
                println!("{q}");
            } else {
                println!("Native question but no SQL found in dataset_query.native.query");
            }
        } else {
            println!("This is a {query_type} question, no native SQL available.");
        }
        return Ok(());
    }

    println!("ID:            {}", card.id);
    println!("Name:          {}", card.name);
    println!(
        "Query Type:    {}",
        card.query_type.as_deref().unwrap_or("-")
    );
    println!(
        "Database ID:   {}",
        card.database_id.map_or("-".into(), |id| id.to_string())
    );
    println!(
        "Collection ID: {}",
        card.collection_id.map_or("-".into(), |id| id.to_string())
    );
    println!("Archived:      {}", card.archived);
    println!("Updated At:    {}", card.updated_at);
    println!(
        "Description:   {}",
        card.description.as_deref().unwrap_or("-")
    );
    Ok(())
}
