/* Copyright © 2025, CosmicMind, Inc. */

use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_aws::trace::{XrayIdGenerator, XrayPropagator};
use opentelemetry_sdk::trace::{self, SdkTracerProvider};
use opentelemetry_sdk::Resource;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::error::TelemetryError;
use crate::exporter::JsonExporter;

/// TelemetryConfig holds configuration information for initializing tracing.
pub struct TelemetryConfig {
    /// The name of your service.
    pub service_name: String,
    /// The version of your service.
    pub service_version: String,
    /// The deployment environment (e.g., "development", "production").
    pub deployment_env: String,
    /// The log level (e.g., "info", "debug").
    pub log_level: String,
}

#[derive(Debug)]
pub struct Telemetry {
    tracer_provider: SdkTracerProvider,
}

impl Telemetry {
    /// Creates a new Telemetry instance with a tracer provider.
    pub fn new(tracer_provider: SdkTracerProvider) -> Self {
        Self { tracer_provider }
    }

    /// Initialize with default configuration.
    pub fn init(config: TelemetryConfig) -> Result<Self, TelemetryError> {
        // Set the global X-Ray propagator.
        Self::init_propagator();
        // Build the tracer provider with the provided config.
        let tracer_provider = Self::init_provider(&config)?;
        // Set up the global subscriber using the validated config.
        Self::init_subscriber(&config, &tracer_provider)?;

        Ok(Self { tracer_provider })
    }

    /// Initialize the global X-Ray propagator.
    pub fn init_propagator() {
        global::set_text_map_propagator(XrayPropagator::default());
    }

    /// Initialize a tracer provider with the given configuration.
    /// All configuration checks are done here.
    pub fn init_provider(config: &TelemetryConfig) -> Result<SdkTracerProvider, TelemetryError> {
        if config.service_name.is_empty() {
            return Err(TelemetryError::InvalidConfiguration(
                "service_name cannot be empty".into(),
            ));
        }
        if config.service_version.is_empty() {
            return Err(TelemetryError::InvalidConfiguration(
                "service_version cannot be empty".into(),
            ));
        }
        if config.deployment_env.is_empty() {
            return Err(TelemetryError::InvalidConfiguration(
                "deployment_env cannot be empty".into(),
            ));
        }

        let resource = Resource::builder().with_attributes(vec![
            KeyValue::new("service.name", config.service_name.clone()),
            KeyValue::new("service.version", config.service_version.clone()),
            KeyValue::new("deployment.environment", config.deployment_env.clone()),
        ]);

        let provider = SdkTracerProvider::builder()
            .with_id_generator(XrayIdGenerator::default())
            .with_sampler(trace::Sampler::AlwaysOn)
            .with_simple_exporter(JsonExporter::new(config.deployment_env.clone()))
            .with_resource(resource.build())
            .build();

        Ok(provider)
    }

    /// Initialize the global subscriber with the given configuration and tracer provider.
    pub fn init_subscriber(
        config: &TelemetryConfig,
        provider: &SdkTracerProvider,
    ) -> Result<(), TelemetryError> {
        let tracer = provider.tracer(config.service_name.clone());
        let subscriber = Registry::default()
            .with(OpenTelemetryLayer::new(tracer))
            .with(EnvFilter::new(&config.log_level));

        tracing::subscriber::set_global_default(subscriber)
            .map_err(|e| TelemetryError::SubscriberError(e.to_string()))
    }

    /// Get a reference to the underlying tracer provider.
    pub fn provider(&self) -> &SdkTracerProvider {
        &self.tracer_provider
    }

    /// Shuts down the tracer provider, flushing any remaining spans.
    pub fn shutdown(&self) {
        let _ = self.tracer_provider.shutdown();
    }
}

impl Drop for Telemetry {
    fn drop(&mut self) {
        let _ = self.tracer_provider.shutdown();
    }
}
