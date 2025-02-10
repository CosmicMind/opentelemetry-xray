/* Copyright © 2025, CosmicMind, Inc. */

#[cfg(test)]
mod tests {
    use crate::exporter::JsonExporter;
    use crate::exporter::{generate_span_id, generate_trace_id};
    use opentelemetry::trace::{SpanContext, SpanId, Status, TraceFlags, TraceState};
    use opentelemetry::InstrumentationScope;
    use opentelemetry_sdk::export::trace::SpanData;
    use opentelemetry_sdk::trace::{SpanEvents, SpanLinks};
    use std::time::SystemTime;

    fn create_test_span() -> SpanData {
        SpanData {
            span_context: SpanContext::new(
                generate_trace_id(),
                generate_span_id(),
                TraceFlags::SAMPLED,
                false,
                TraceState::default(),
            ),
            parent_span_id: SpanId::INVALID,
            span_kind: opentelemetry::trace::SpanKind::Server,
            name: "test-span".into(),
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            attributes: Vec::new(),
            events: SpanEvents::default(),
            links: SpanLinks::default(),
            status: Status::Ok,
            dropped_attributes_count: 0,
            instrumentation_scope: InstrumentationScope::default(),
        }
    }

    #[test]
    fn test_basic_export() {
        let mut exporter = JsonExporter::new("test".to_string());
        let span = create_test_span();
        let result = exporter.export_batch(vec![span]);
        assert!(result.is_ok());
    }
}
