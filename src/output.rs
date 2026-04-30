use std::io::{self, Write};

use anyhow::{Context, Result};
use serde_json::{Map, Value};
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
            Value::String(s) => s,
            Value::Null => "NULL".to_string(),
            other => other.to_string(),
        }));
    }
    let mut table = builder.build();
    table.with(Style::rounded());
    println!("{table}");
}

pub fn print_query_csv(result: QueryResult) -> Result<()> {
    let mut writer = csv::Writer::from_writer(io::stdout().lock());
    writer
        .write_record(result.data.cols.iter().map(|c| c.name.as_str()))
        .context("failed to write csv header")?;
    for row in result.data.rows {
        let cells: Vec<String> = row.into_iter().map(csv_cell).collect();
        writer.write_record(&cells).context("failed to write csv row")?;
    }
    writer.flush().context("failed to flush csv output")
}

pub fn print_query_json(result: QueryResult) -> Result<()> {
    let names: Vec<String> = result.data.cols.into_iter().map(|c| c.name).collect();
    let records: Vec<Map<String, Value>> = result
        .data
        .rows
        .into_iter()
        .map(|row| {
            names
                .iter()
                .cloned()
                .zip(row.into_iter().chain(std::iter::repeat(Value::Null)))
                .collect()
        })
        .collect();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer(&mut handle, &records).context("failed to write json")?;
    handle.write_all(b"\n").context("failed to write newline")
}

fn csv_cell(v: Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::String(s) => s,
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}
