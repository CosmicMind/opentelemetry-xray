use thiserror::Error;

/// TelemetryError encapsulates errors encountered during telemetry initialization
/// or operation.
#[derive(Debug, Error)]
pub enum TelemetryError {
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Tracing subscriber error: {0}")]
    SubscriberError(String),
}
