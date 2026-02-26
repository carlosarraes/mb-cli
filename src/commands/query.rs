use anyhow::Result;

use crate::client::MetabaseClient;
use crate::output;

pub fn run(client: &MetabaseClient, database: &str, sql: &str, json: bool, csv: bool) -> Result<()> {
    let db_id = client.resolve_database(database)?;

    if csv {
        let raw = client.export_query(db_id, sql, "csv")?;
        print!("{raw}");
        return Ok(());
    }

    if json {
        let raw = client.export_query(db_id, sql, "json")?;
        println!("{raw}");
        return Ok(());
    }

    let result = client.run_query(db_id, sql)?;
    output::print_query_table(&result);
    Ok(())
}
