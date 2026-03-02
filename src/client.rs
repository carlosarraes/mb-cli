use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::config::Config;

pub struct MetabaseClient {
    http: Client,
    base_url: String,
    auth: Auth,
}

enum Auth {
    ApiKey(String),
    Session(String),
}

#[derive(Deserialize)]
pub struct Database {
    pub id: i64,
    pub name: String,
    pub engine: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DatabaseListResponse {
    Wrapped { data: Vec<Database> },
    Bare(Vec<Database>),
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct DatabaseMetadata {
    pub id: i64,
    pub name: String,
    pub tables: Vec<Table>,
}

#[derive(Deserialize)]
pub struct Table {
    pub id: i64,
    pub name: String,
    pub schema: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct TableMetadata {
    pub id: i64,
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Deserialize)]
pub struct Field {
    pub id: i64,
    pub name: String,
    pub base_type: String,
    pub database_type: String,
    pub semantic_type: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryResult {
    pub data: QueryData,
}

#[derive(Deserialize)]
pub struct QueryData {
    pub cols: Vec<QueryCol>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

#[derive(Deserialize)]
pub struct QueryCol {
    pub name: String,
    #[allow(dead_code)]
    pub base_type: String,
}

impl MetabaseClient {
    pub fn new(config: &Config) -> Result<Self> {
        let mb = &config.metabase;
        let auth = match (&mb.api_key, &mb.session_token) {
            (Some(key), _) => Auth::ApiKey(key.clone()),
            (_, Some(token)) => Auth::Session(token.clone()),
            _ => bail!("no api_key or session_token in config\nRun `mb config` to set up"),
        };
        Ok(Self {
            http: Client::new(),
            base_url: mb.url.trim_end_matches('/').to_string(),
            auth,
        })
    }

    fn auth_header(&self) -> (&str, &str) {
        match &self.auth {
            Auth::ApiKey(key) => ("x-api-key", key),
            Auth::Session(token) => ("X-Metabase-Session", token),
        }
    }

    fn check_response(resp: reqwest::blocking::Response) -> Result<reqwest::blocking::Response> {
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().unwrap_or_default();
            bail!("Metabase API error ({status}): {}", &body[..body.len().min(500)]);
        }
        Ok(resp)
    }

    fn get(&self, path: &str) -> Result<reqwest::blocking::Response> {
        let url = format!("{}{}", self.base_url, path);
        let (header, value) = self.auth_header();
        let resp = self.http.get(&url)
            .header(header, value)
            .send()
            .with_context(|| format!("failed to reach Metabase at {url}"))?;
        Self::check_response(resp)
    }

    fn post_json(&self, path: &str, body: &serde_json::Value) -> Result<reqwest::blocking::Response> {
        let url = format!("{}{}", self.base_url, path);
        let (header, value) = self.auth_header();
        let resp = self.http.post(&url)
            .header(header, value)
            .json(body)
            .send()
            .with_context(|| format!("failed to reach Metabase at {url}"))?;
        Self::check_response(resp)
    }

    pub fn list_databases(&self) -> Result<Vec<Database>> {
        let resp = self.get("/api/database")?;
        let wrapper: DatabaseListResponse = resp.json().context("failed to parse database list")?;
        Ok(match wrapper {
            DatabaseListResponse::Wrapped { data } => data,
            DatabaseListResponse::Bare(data) => data,
        })
    }

    pub fn database_metadata(&self, id: i64) -> Result<DatabaseMetadata> {
        let resp = self.get(&format!("/api/database/{id}/metadata"))?;
        resp.json().context("failed to parse database metadata")
    }

    pub fn table_query_metadata(&self, id: i64) -> Result<TableMetadata> {
        let resp = self.get(&format!("/api/table/{id}/query_metadata"))?;
        resp.json().context("failed to parse table metadata")
    }

    fn query_body(db_id: i64, sql: &str) -> serde_json::Value {
        serde_json::json!({
            "database": db_id,
            "type": "native",
            "native": {
                "query": sql,
                "template-tags": {}
            },
            "parameters": []
        })
    }

    pub fn run_query(&self, db_id: i64, sql: &str) -> Result<QueryResult> {
        let body = Self::query_body(db_id, sql);
        let resp = self.post_json("/api/dataset", &body)?;
        resp.json().context("failed to parse query result")
    }

    pub fn export_query(&self, db_id: i64, sql: &str, format: &str) -> Result<String> {
        let body = Self::query_body(db_id, sql);
        let resp = self.post_json(&format!("/api/dataset/{format}"), &body)?;
        resp.text().context("failed to read export response")
    }

    pub fn resolve_database(&self, input: &str) -> Result<i64> {
        if let Ok(id) = input.parse::<i64>() {
            return Ok(id);
        }
        let databases = self.list_databases()?;
        let items: Vec<_> = databases.iter().map(|db| (db.id, db.name.as_str())).collect();
        resolve_by_name(input, &items, "database")
    }

    pub fn resolve_table(&self, db_id: i64, input: &str) -> Result<i64> {
        if let Ok(id) = input.parse::<i64>() {
            return Ok(id);
        }
        let metadata = self.database_metadata(db_id)?;
        let items: Vec<_> = metadata.tables.iter().map(|t| (t.id, t.name.as_str())).collect();
        resolve_by_name(input, &items, "table")
    }
}

fn resolve_by_name(input: &str, items: &[(i64, &str)], entity: &str) -> Result<i64> {
    let matches: Vec<_> = items.iter()
        .filter(|(_, name)| name.eq_ignore_ascii_case(input))
        .collect();
    match matches.len() {
        0 => bail!("no {entity} found matching '{input}'"),
        1 => Ok(matches[0].0),
        _ => bail!(
            "multiple {entity}s match '{input}', use ID instead: {}",
            matches.iter().map(|(id, name)| format!("{name} (id: {id})")).collect::<Vec<_>>().join(", ")
        ),
    }
}
