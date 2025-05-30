use crate::config::Config;
use crate::model::AuditEvent;
use crate::partition::build_partition_path;
use anyhow::Result;
use std::fs::{create_dir_all, File};
use std::io::Write;
use serde_json::to_writer_pretty;

pub fn write_batches(cfg: &Config, events: &[AuditEvent]) -> Result<()> {
    for (path, batch) in group_by_partition(cfg, events)? {
        create_dir_all(path.parent().unwrap())?;
        let mut file = File::create(path)?;

        if cfg.output.format == "ndjson" {
            for event in batch {
                writeln!(&mut file, "{}", serde_json::to_string(&event)?)?;

            }
        } else {
            to_writer_pretty(&file, &batch)?;
        }
    }
    Ok(())
}

use std::collections::HashMap;
use std::path::PathBuf;

fn group_by_partition(cfg: &Config, events: &[AuditEvent]) -> Result<HashMap<PathBuf, Vec<AuditEvent>>> {
    let mut map: HashMap<PathBuf, Vec<AuditEvent>> = HashMap::new();
    for event in events {
        let path = build_partition_path(&cfg.output.directory, event)?;
        map.entry(path).or_insert_with(Vec::new).push(event.clone());
    }
    Ok(map)
}
