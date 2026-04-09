//! Metrics System.

pub struct Metrics;

impl Metrics {
    pub fn new() -> Self {
        Self
    }

    pub fn increment_counter(&self, name: &'static str) {
        metrics::counter!(name).increment(1);
    }

    pub fn record_histogram(&self, name: &'static str, value: f64) {
        metrics::histogram!(name).record(value);
    }

    pub fn set_gauge(&self, name: &'static str, value: f64) {
        metrics::gauge!(name).set(value);
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
