/* Copyright © 2025, CosmicMind, Inc. */

use http::{HeaderMap, HeaderName, HeaderValue};
use opentelemetry::{
    context::Context,
    global,
    propagation::{Extractor, Injector},
};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::get_x_amzn_trace_id;

/// HeaderInjector is a helper for propagating tracing headers.
pub struct HeaderInjector<'a>(pub &'a mut HeaderMap);

impl Injector for HeaderInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        if key.eq_ignore_ascii_case("traceparent") {
            // Instead of inserting "traceparent", use the AWS X‑Ray header key.
            // Using HeaderName::from_static is simpler and ensures a canonical key.
            let header_name = HeaderName::from_static(get_x_amzn_trace_id());
            if let Ok(header_value) = HeaderValue::from_str(&value) {
                self.0.insert(header_name, header_value);
            }
        } else if let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) {
            if let Ok(header_value) = HeaderValue::from_str(&value) {
                self.0.insert(header_name, header_value);
            }
        }
    }
}

/// HeaderExtractor is a helper for extracting headers for tracing propagation.
pub struct HeaderExtractor<'a>(pub &'a HeaderMap);

impl Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

/// inject_headers injects the current OpenTelemetry tracing context into the given headers.
pub fn inject_headers(headers: &mut HeaderMap) {
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(
            &tracing::Span::current().context(),
            &mut HeaderInjector(headers),
        );
    });
}

/// Extracts the current OpenTelemetry tracing context from the given headers.
/// Returns the extracted Context.
pub fn extract_headers(headers: &HeaderMap) -> Context {
    global::get_text_map_propagator(|propagator| propagator.extract(&HeaderExtractor(headers)))
}
