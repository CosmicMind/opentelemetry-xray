/* Copyright © 2025, CosmicMind, Inc. */

use std::sync::OnceLock;

mod headers;
mod provider;
#[cfg(test)]
mod tests;

pub use headers::{extract_headers, inject_headers, HeaderExtractor, HeaderInjector};
pub use provider::{Telemetry, TelemetryConfig};

static X_AMZN_TRACE_ID: OnceLock<&'static str> = OnceLock::new();

pub fn get_x_amzn_trace_id() -> &'static str {
    X_AMZN_TRACE_ID.get_or_init(|| "x-amzn-trace-id")
}
