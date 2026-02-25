//! FemtoClaw Observability Library.
//!
//! Provides telemetry, logging, metrics, and audit trail according to
//! FemtoClaw Observability Specification (FC-08).

pub mod event;
pub mod logger;
pub mod metrics;

pub use event::{Event, EventType};
pub use logger::Logger;
pub use metrics::Metrics;
