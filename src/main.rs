mod config;
mod db;
mod model;
mod writer;
mod partition;
mod utils;

use anyhow::Result;
use crate::config::load_config;
use crate::db::fetch_audit_events;
use crate::writer::write_batches;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = load_config("config/config.yaml")?;
    let events = fetch_audit_events(&cfg).await?;
    write_batches(&cfg, &events)?;
    Ok(())
}
