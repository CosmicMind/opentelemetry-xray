# opentelemetry-xray

opentelemetry-xray is a Rust library that provides a flexible interface for AWS X‑Ray distributed tracing using OpenTelemetry. It simplifies the integration of AWS X‑Ray with your applications by setting up a global tracer provider, mapping tracing contexts into AWS X‑Ray–compatible formats, and supporting rich span information such as annotations, metadata, exceptions, and HTTP context.

## Features

- **AWS X‑Ray Integration:**
    - Formats trace IDs according to AWS X‑Ray standards.
    - Transforms the standard `traceparent` header into the AWS X‑Ray header (`x-amzn-trace-id`).
    - Exports span data as JSON segments that AWS X‑Ray can understand.
- **OpenTelemetry & Tracing Support:**
    - Leverages the robust OpenTelemetry ecosystem.
    - Integrates seamlessly with the `tracing` crate for structured logging and span management.
- **Span Enrichment:**
    - **Annotations:** Indexed fields (e.g. use the prefix `annotation.`) for filtering and searching.
    - **Metadata:** Additional non-indexed data (using the `metadata.` prefix).
    - **Exceptions:** Detailed error information (with keys like `exception.type` and `exception.message`).
    - **HTTP Context:** Record HTTP request and response details (using keys such as `http.request.method` and `http.response.status`).

> **Note:** In these examples we show keys as dot‑separated (e.g. `annotation.otel.kind`) without extra quotation marks. In your actual Rust code, if a field name isn't a valid identifier (because it includes a period), you may need to use a string literal (for example, `"annotation.otel.kind" = "server"`)—the important part is that the exported key appears as `annotation.otel.kind` (and similarly for `http.request`, `http.response`, etc).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
opentelemetry-xray = "0.1.5"
opentelemetry = { version = "0.29.0", features = ["trace"] }
tracing = "0.1.41"
tracing-subscriber = { version = "0.3.19", features = ["env-filter", "json"] }
```

## Quick Start

Initialize your telemetry and create a span enriched with annotations, metadata, and exception details:

```rust
use opentelemetry_xray::{Telemetry, TelemetryConfig};
use tracing::{info_span, error, Instrument};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = TelemetryConfig {
        service_name: "my-service".into(),
        service_version: env!("CARGO_PKG_VERSION").into(),
        deployment_env: "production".into(),
        log_level: "info".into(),
    };
    
    // Initialize the propagator, tracer provider, and subscriber.
    let telemetry = Telemetry::init(config)?;
    
    // Create a span with enriched attributes. In these examples, we conceptually use keys like:
    //   annotation.user_id, metadata.request_id, http.request.method, etc.
    let span = info_span!(
        main_operation,
        annotation.user_id = 42,
        metadata.request_id = "abc123",
        // HTTP-related attributes:
        http.request.method = "GET",
        http.request.url = "https://example.com/api"
    );
    
    async {
        if let Err(e) = do_something().await {
            // Record exception details:
            error!(
                exception.type = "OperationError",
                exception.message = %e,
                "Operation failed"
            );
        }
    }
    .instrument(span)
    .await;
    
    telemetry.shutdown();
    
    Ok(())
}

async fn do_something() -> Result<(), &'static str> {
    Err("Something went wrong")
}
```

## Using Annotations, Metadata, and Exceptions

### Annotations

Annotations are meant to be indexed and are useful for filtering and searching in AWS X‑Ray. Specify annotation attributes using the `annotation.` prefix:

```rust
use tracing::info_span;

let span = info_span!(
    processing_order,
    annotation.order_id = 1001,
    annotation.priority = "high"
);
```

### Metadata

Metadata provides additional context but is not indexed. Use the `metadata.` prefix:

```rust
let span = info_span!(
    user_session,
    metadata.session_id = "xyz789",
    metadata.user_agent = "Mozilla/5.0"
);
```

### Exception Reporting

When errors occur, capture detailed error information by using the `exception.` prefix:

```rust
use tracing::error;

let err = "Null Pointer Exception";
error!(
    exception.type = "NullPointerException",
    exception.message = err,
    "An error occurred"
);
```

## HTTP Integration

### Server-side Tracing

Extract incoming trace contexts from HTTP requests and create spans enriched with HTTP attributes:

```rust
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use opentelemetry_xray::{extract_headers, Telemetry};
use axum::http::Request;
use axum::body::Body;

async fn handler() {
    // Your request handling logic.
}

async fn build_router() -> Router {
    Router::new()
        .route("/", get(handler))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let span = tracing::info_span!(
                    http_request,
                    http.request.method = ?request.method(),
                    http.request.url = ?request.uri()
                );
                // Extract and set the incoming trace context:
                let parent_context = extract_headers(request.headers());
                span.set_parent(parent_context);
                span
            })
        )
}
```

### Client-side Tracing

Inject the current trace context into outgoing HTTP requests so downstream services continue the distributed trace:

```rust
use reqwest::Client;
use http::HeaderMap;
use opentelemetry_xray::inject_headers;

async fn make_request(client: &Client, url: &str) -> Result<(), reqwest::Error> {
    let mut headers = HeaderMap::new();
    // Inject the current OpenTelemetry context (this converts the standard
    // `traceparent` header to the AWS X‑Ray header, e.g. `x-amzn-trace-id`).
    inject_headers(&mut headers);
    
    client.get(url)
        .headers(headers)
        .send()
        .await?;
    
    Ok(())
}
```

## Configuration Options

Configure telemetry using the `TelemetryConfig` struct:

```rust
let config = TelemetryConfig {
    service_name: "my-service".into(),
    service_version: env!("CARGO_PKG_VERSION").into(),
    deployment_env: "production".into(),
    log_level: "info".into(),
};
```

## Best Practices

1. **Initialization:**
    - Use `Telemetry::init()` for a standardized setup of the propagator, tracer provider, and subscriber.
    - Always call `telemetry.shutdown()` at application exit to ensure all spans are flushed.
2. **Span Enrichment:**
    - Use `tracing` macros (e.g., `info_span!`, `error!`) to create spans.
    - Use meaningful dot‑separated keys (e.g., `annotation.`, `metadata.`, `http.request.`, `exception.`) to indicate the purpose of each attribute.
3. **Error Handling:**
    - Capture and report exceptions using the designated keys to facilitate troubleshooting.
4. **HTTP Instrumentation:**
    - For server-side tracing, extract HTTP headers via `extract_headers`.
    - For client-side requests, inject the trace context with `inject_headers`.

## Testing

Run the test suite:

```bash
cargo test
```

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your improvements or bug fixes.

## Related Podcasts

- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust)
- [AWS X-Ray](https://aws.amazon.com/xray/)
- [tracing](https://github.com/tokio-rs/tracing)

## Support

For issues and feature requests, please use the [GitHub issue tracker](https://github.com/cosmicmind/opentelemetry-xray/issues).