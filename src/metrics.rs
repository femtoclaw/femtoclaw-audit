//! Metrics System.

use metrics::{Counter, Gauge, Histogram};

pub struct Metrics;

impl Metrics {
    pub fn new() -> Self {
        Self
    }

    pub fn increment_counter(&self, name: &str) {
        metrics::counter!(name).increment(1);
    }

    pub fn record_histogram(&self, name: &str, value: f64) {
        metrics::histogram!(name).record(value);
    }

    pub fn set_gauge(&self, name: &str, value: f64) {
        metrics::gauge!(name).set(value);
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
