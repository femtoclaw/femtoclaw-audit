//! Telemetry - Unified Observability Interface.
//!
//! Provides a unified interface for all observability operations.

use crate::{Audit, Event, Logger, Metrics};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::VecDeque;
use uuid::Uuid;

const MAX_EVENTS: usize = 1000;

pub struct Telemetry {
    logger: Logger,
    metrics: Metrics,
    audit: Audit,
    events: Arc<RwLock<VecDeque<Event>>>,
}

impl Telemetry {
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
            metrics: Metrics::new(),
            audit: Audit::new(),
            events: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_EVENTS))),
        }
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub fn audit(&self) -> &Audit {
        &self.audit
    }

    pub async fn emit(&self, event: Event) {
        let mut events = self.events.write().await;
        if events.len() >= MAX_EVENTS {
            events.pop_front();
        }
        events.push_back(event);
    }

    pub async fn emit_and_log(&self, event: Event) {
        self.emit(event.clone()).await;
        self.logger.log(
            tracing::Level::INFO,
            "femtoclaw",
            &format!("{:?}", event.event_type),
            event.payload,
        );
    }

    pub async fn get_events(&self) -> Vec<Event> {
        let events = self.events.read().await;
        events.iter().cloned().collect()
    }

    pub async fn start_trace(&self) -> Uuid {
        Uuid::new_v4()
    }
}

impl Default for Telemetry {
    fn default() -> Self {
        Self::new()
    }
}
