use crate::model::AuditEvent;
use anyhow::Result;
use chrono::{Datelike, Timelike};
use std::path::PathBuf;

pub fn build_partition_path(base: &str, event: &AuditEvent) -> Result<PathBuf> {
    let ts = event.changed_at;
    let path = PathBuf::from(base)
        .join(&event.schema_name)
        .join(&event.table_name)
        .join(format!("{:04}/{:02}/{:02}/{:02}", ts.year(), ts.month(), ts.day(), ts.hour()));
    let filename = format!("audit_{}.json", ts.format("%Y%m%dT%H%M%S"));
    Ok(path.join(filename))
}
