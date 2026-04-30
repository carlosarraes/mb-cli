use anyhow::Result;

use crate::client::MetabaseClient;
use crate::output;

pub fn run(
    client: &MetabaseClient,
    database: &str,
    sql: &str,
    json: bool,
    csv: bool,
) -> Result<()> {
    let db_id = client.resolve_database(database)?;
    let result = client.run_query(db_id, sql)?;

    if csv {
        return output::print_query_csv(result);
    }
    if json {
        return output::print_query_json(result);
    }
    output::print_query_table(result);
    Ok(())
}
