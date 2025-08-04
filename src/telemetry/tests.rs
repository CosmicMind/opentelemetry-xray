/* Copyright © 2025, CosmicMind, Inc. */

#[cfg(test)]
mod tests {
    use crate::error::TelemetryError;
    use crate::telemetry::{
        get_x_amzn_trace_id, inject_headers, HeaderExtractor, HeaderInjector, Telemetry,
        TelemetryConfig,
    };
    use http::{HeaderMap, HeaderName, HeaderValue};
    use opentelemetry::propagation::{Extractor, Injector};
    use tracing;

    fn setup_test_config() -> TelemetryConfig {
        TelemetryConfig {
            service_name: "test-service".to_string(),
            service_version: "0.1.0".to_string(),
            deployment_env: "test".to_string(),
            log_level: "debug".to_string(),
        }
    }

    #[test]
    fn test_telemetry_initialization() {
        let config = setup_test_config();

        // Test individual components
        Telemetry::init_propagator();
        let provider = Telemetry::init_provider(&config).expect("Failed to initialize provider");

        // Create telemetry without subscriber for testing
        let telemetry = Telemetry::new(provider);
        telemetry.shutdown();
    }

    #[test]
    fn test_header_injection() {
        let config = setup_test_config();

        // Initialize the global propagator and create the provider.
        Telemetry::init_propagator();
        let provider = Telemetry::init_provider(&config).expect("Failed to initialize provider");
        // Initialize the subscriber so spans carry proper OpenTelemetry context.
        Telemetry::init_subscriber(&config, &provider).expect("Failed to initialize subscriber");
        let telemetry = Telemetry::new(provider);

        // Create a parent span using tracing's macros so that it becomes the active span.
        let parent_span = tracing::info_span!("parent_span");
        let parent_trace_value = {
            let _parent_guard = parent_span.enter();
            let mut parent_headers = http::HeaderMap::new();
            inject_headers(&mut parent_headers);
            assert!(parent_headers.contains_key(get_x_amzn_trace_id()));
            parent_headers
                .get(get_x_amzn_trace_id())
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        };

        // Create a child span in the context of the parent.
        let child_span = tracing::info_span!("child_span");
        let child_trace_value = {
            let _child_guard = child_span.enter();
            let mut child_headers = http::HeaderMap::new();
            inject_headers(&mut child_headers);
            assert!(child_headers.contains_key(get_x_amzn_trace_id()));
            child_headers
                .get(get_x_amzn_trace_id())
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        };

        // Verify that the header values differ between parent and child spans.
        assert_ne!(
            parent_trace_value, child_trace_value,
            "Child should have a different header value than the parent"
        );

        telemetry.shutdown();
    }

    #[test]
    fn test_header_extraction() {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-amzn-trace-id"),
            HeaderValue::from_static(
                "Root=1-5759e988-bd862e3fe1be46a994272793;Parent=53995c3f42cd8ad8;Sampled=1",
            ),
        );

        let extractor = HeaderExtractor(&headers);
        assert_eq!(
            extractor.get("x-amzn-trace-id"),
            Some("Root=1-5759e988-bd862e3fe1be46a994272793;Parent=53995c3f42cd8ad8;Sampled=1")
        );
    }

    #[test]
    fn test_invalid_service_name() {
        let mut config = setup_test_config();
        config.service_name = "".to_string();

        let result = Telemetry::init_provider(&config);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(err, TelemetryError::InvalidConfiguration(_)));
        }
    }

    #[test]
    fn test_propagator_conversion() {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("traceparent"),
            HeaderValue::from_static("00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01"),
        );

        let mut output_headers = HeaderMap::new();
        let mut injector = HeaderInjector(&mut output_headers);
        injector.set(
            "traceparent",
            "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".to_string(),
        );

        assert!(!output_headers.contains_key("traceparent"));
        assert!(output_headers.contains_key(get_x_amzn_trace_id()));
    }
}
