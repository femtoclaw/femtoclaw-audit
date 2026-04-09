//! FemtoClaw Observability Library.
//!
//! Provides telemetry, logging, metrics, and audit trail according to
//! FemtoClaw Observability Specification (FC-08).
//!
//! # Architecture
//!
//! - [`Telemetry`] - Main entry point for all observability operations
//! - [`Event`] - Structured telemetry events
//! - [`Logger`] - Structured logging
//! - [`Metrics`] - Metrics collection
//! - [`Audit`] - Audit trail

pub mod audit;
pub mod event;
pub mod logger;
pub mod metrics;
pub mod telemetry;

pub use audit::Audit;
pub use event::{Event, EventType};
pub use logger::Logger;
pub use metrics::Metrics;
pub use telemetry::Telemetry;
