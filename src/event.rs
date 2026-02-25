//! Telemetry Event Definitions.
//!
//! Telemetry Event System records structured execution events.
//! Events MUST be emitted for:
//! - Input received
//! - Protocol validated
//! - Authorization decision
//! - Capability execution start
//! - Capability execution complete
//! - Memory write
//! - Execution completion
//! - Execution failure

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub payload: serde_json::Value,
    pub trace_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    InputReceived,
    ProtocolValidated,
    ProtocolRejected,
    AuthorizationDecision,
    CapabilityExecutionStart,
    CapabilityExecutionComplete,
    CapabilityExecutionError,
    MemoryWrite,
    ExecutionComplete,
    ExecutionError,
    StateTransition,
}

impl Event {
    pub fn new(event_type: EventType, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            payload,
            trace_id: None,
        }
    }

    pub fn with_trace(mut self, trace_id: Uuid) -> Self {
        self.trace_id = Some(trace_id);
        self
    }

    pub fn input_received(input: &str) -> Self {
        Self::new(
            EventType::InputReceived,
            serde_json::json!({ "input": input }),
        )
    }

    pub fn protocol_validated() -> Self {
        Self::new(
            EventType::ProtocolValidated,
            serde_json::json!({ "result": "valid" }),
        )
    }

    pub fn protocol_rejected(reason: &str) -> Self {
        Self::new(
            EventType::ProtocolRejected,
            serde_json::json!({ "reason": reason }),
        )
    }

    pub fn authorization_decision(tool: &str, decision: &str) -> Self {
        Self::new(
            EventType::AuthorizationDecision,
            serde_json::json!({
                "tool": tool,
                "decision": decision
            }),
        )
    }

    pub fn capability_execution_start(tool: &str, args: &serde_json::Value) -> Self {
        Self::new(
            EventType::CapabilityExecutionStart,
            serde_json::json!({
                "tool": tool,
                "args": args
            }),
        )
    }

    pub fn capability_execution_complete(tool: &str, result: &str) -> Self {
        Self::new(
            EventType::CapabilityExecutionComplete,
            serde_json::json!({
                "tool": tool,
                "result": result
            }),
        )
    }

    pub fn capability_execution_error(tool: &str, error: &str) -> Self {
        Self::new(
            EventType::CapabilityExecutionError,
            serde_json::json!({
                "tool": tool,
                "error": error
            }),
        )
    }

    pub fn execution_complete() -> Self {
        Self::new(EventType::ExecutionComplete, serde_json::json!({}))
    }

    pub fn execution_error(error: &str) -> Self {
        Self::new(
            EventType::ExecutionError,
            serde_json::json!({ "error": error }),
        )
    }
}
