/* Copyright © 2025, CosmicMind, Inc. */

//! opentelemetry-xray provides a flexible interface for AWS X‑Ray distributed tracing using OpenTelemetry.

pub mod error;
pub mod exporter;
pub mod telemetry;

// Re-export key types for users.
pub use error::TelemetryError;
pub use exporter::JsonExporter;
pub use telemetry::{
    extract_headers, inject_headers, HeaderExtractor, HeaderInjector, Telemetry, TelemetryConfig,
};
