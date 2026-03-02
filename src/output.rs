use tabled::{Table, Tabled, builder::Builder, settings::Style};

use crate::client::QueryResult;

pub fn print_table<T: Tabled>(items: &[T]) {
    if items.is_empty() {
        println!("No results.");
        return;
    }
    let mut table = Table::new(items);
    table.with(Style::rounded());
    println!("{table}");
}

pub fn print_query_table(result: QueryResult) {
    if result.data.rows.is_empty() {
        println!("No results.");
        return;
    }
    let mut builder = Builder::default();
    builder.push_record(result.data.cols.into_iter().map(|c| c.name));
    for row in result.data.rows {
        builder.push_record(row.into_iter().map(|v| match v {
            serde_json::Value::String(s) => s,
            serde_json::Value::Null => "NULL".to_string(),
            other => other.to_string(),
        }));
    }
    let mut table = builder.build();
    table.with(Style::rounded());
    println!("{table}");
}
