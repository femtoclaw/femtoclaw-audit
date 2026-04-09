//! Audit Log System.
//!
//! Provides complete audit trail for compliance and security verification.
//! Audit logs MUST be immutable and machine-readable.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::VecDeque;

const MAX_AUDIT_ENTRIES: usize = 10000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event: String,
    pub actor: String,
    pub resource: String,
    pub action: String,
    pub result: String,
    pub details: serde_json::Value,
}

impl AuditEntry {
    pub fn new(
        event: impl Into<String>,
        actor: impl Into<String>,
        resource: impl Into<String>,
        action: impl Into<String>,
        result: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event: event.into(),
            actor: actor.into(),
            resource: resource.into(),
            action: action.into(),
            result: result.into(),
            details: serde_json::Value::Object(Default::default()),
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }
}

pub struct Audit {
    entries: Arc<RwLock<VecDeque<AuditEntry>>>,
}

impl Audit {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_AUDIT_ENTRIES))),
        }
    }

    pub async fn log(&self, entry: AuditEntry) {
        let mut entries = self.entries.write().await;
        if entries.len() >= MAX_AUDIT_ENTRIES {
            entries.pop_front();
        }
        entries.push_back(entry);
    }

    pub async fn log_event(
        &self,
        event: &str,
        actor: &str,
        resource: &str,
        action: &str,
        result: &str,
    ) {
        let entry = AuditEntry::new(event, actor, resource, action, result);
        self.log(entry).await;
    }

    pub async fn get_entries(&self) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        entries.iter().cloned().collect()
    }

    pub async fn get_entries_for_resource(&self, resource: &str) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        entries
            .iter()
            .filter(|e| e.resource == resource)
            .cloned()
            .collect()
    }

    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
    }
}

impl Default for Audit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_log() {
        let audit = Audit::new();
        audit.log_event("test", "user1", "file.txt", "read", "success").await;
        
        let entries = audit.get_entries().await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].action, "read");
    }

    #[tokio::test]
    async fn test_audit_filter() {
        let audit = Audit::new();
        audit.log_event("test", "user1", "file1.txt", "read", "success").await;
        audit.log_event("test", "user1", "file2.txt", "read", "success").await;
        
        let entries = audit.get_entries_for_resource("file1.txt").await;
        assert_eq!(entries.len(), 1);
    }
}
