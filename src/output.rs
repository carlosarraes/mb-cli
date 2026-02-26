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

pub fn print_query_table(result: &QueryResult) {
    if result.data.rows.is_empty() {
        println!("No results.");
        return;
    }
    let mut builder = Builder::default();
    builder.push_record(result.data.cols.iter().map(|c| c.name.clone()));
    for row in &result.data.rows {
        builder.push_record(row.iter().map(|v| match v {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Null => "NULL".to_string(),
            other => other.to_string(),
        }));
    }
    let mut table = builder.build();
    table.with(Style::rounded());
    println!("{table}");
}
