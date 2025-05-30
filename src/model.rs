use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub schema_name: String,
    pub table_name: String,
    pub customer_id: Option<i32>,
    pub change_type: String,
    pub old_data: Option<Value>,
    pub new_data: Option<Value>,
    pub changed_at: DateTime<Utc>,
}
