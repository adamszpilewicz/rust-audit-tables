use tokio_postgres::{NoTls, Row};
use crate::config::Config;
use crate::model::AuditEvent;
use anyhow::Result;

pub async fn fetch_audit_events(cfg: &Config) -> Result<Vec<AuditEvent>> {
    let dsn = format!(
        "host={} port={} user={} password={} dbname={}",
        cfg.database.host, cfg.database.port, cfg.database.user, cfg.database.password, cfg.database.dbname
    );
    let (client, connection) = tokio_postgres::connect(&dsn, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let since = cfg.batch.since;
    let stmt = client.prepare(
        "SELECT schema_name, table_name, customer_id, change_type, old_data, new_data, changed_at \
         FROM audit_customer WHERE changed_at >= $1 ORDER BY changed_at LIMIT $2"
    ).await?;

    let rows = client.query(&stmt, &[&since, &(cfg.batch.max_rows as i64)]).await?;
    let events = rows.into_iter().map(row_to_event).collect();
    Ok(events)
}

fn row_to_event(row: Row) -> AuditEvent {
    AuditEvent {
        schema_name: row.get("schema_name"),
        table_name: row.get("table_name"),
        customer_id: row.get("customer_id"),
        change_type: row.get("change_type"),
        old_data: row.get("old_data"),       // type: Option<serde_json::Value>
        new_data: row.get("new_data"),       // type: Option<serde_json::Value>
        changed_at: row.get("changed_at"),   // type: DateTime<Utc>
    }
}
