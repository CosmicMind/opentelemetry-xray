# opentelemetry-xray

opentelemetry-xray is a Rust library that provides a flexible interface for AWS X‑Ray distributed tracing using OpenTelemetry. It simplifies the integration of AWS X‑Ray with your applications by setting up a global tracer provider, structured logging via `tracing`, and graceful shutdown of the tracer.

## Features

- **AWS X‑Ray Integration:** 
  - Format trace IDs according to AWS X‑Ray standards
  - Automatic propagation of X-Ray trace context
  - Compatible with AWS X-Ray console and analysis tools
- **OpenTelemetry Support:** 
  - Built on top of the robust OpenTelemetry ecosystem
  - Standard span attributes and semantic conventions
  - Extensible trace context propagation
- **Structured Logging:** 
  - Seamless integration with the `tracing` ecosystem
  - Enriched, structured logs with trace context
  - Configurable log levels and filters
- **Modular Design:**
  - Flexible initialization options
  - Component-level control
  - Customizable setup for different environments

## Installation

Add opentelemetry-xray to your `Cargo.toml`:

```toml
[dependencies]
opentelemetry-xray = "0.1.0"
opentelemetry = { version = "0.27", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## Usage

### Quick Start

For most applications, use the default initialization:

```rust
use opentelemetry_xray::{Telemetry, TelemetryConfig};

let config = TelemetryConfig {
    service_name: "my-service".into(),
    service_version: env!("CARGO_PKG_VERSION").into(),
    deployment_env: "production".into(),
    log_level: "info".into(),
};

// Initialize everything with default settings
let telemetry = Telemetry::init(config)?;

// Your application logic here...

// Clean up on shutdown
telemetry.shutdown();
```

### Modular Initialization

For more control, initialize components individually:

```rust
use opentelemetry_xray::{Telemetry, TelemetryConfig};

let config = TelemetryConfig {
    service_name: "my-service".into(),
    service_version: env!("CARGO_PKG_VERSION").into(),
    deployment_env: "production".into(),
    log_level: "info".into(),
};

// Initialize components as needed
Telemetry::init_propagator();  // Set up X-Ray context propagation
let provider = Telemetry::init_provider(&config)?;  // Create tracer provider
Telemetry::init_subscriber(&config, &provider)?;  // Set up tracing subscriber

// Create telemetry instance
let telemetry = Telemetry::new(provider);
```

### Span Creation

Create spans using the `tracing` macros for automatic context propagation:

```rust
use tracing::info_span;

let span = info_span!("my_operation");
let _guard = span.enter();

// Operation is now traced with X-Ray
perform_work();
```

### Context Propagation

Propagate trace context across service boundaries:

```rust
use opentelemetry_xray::{inject_headers, extract_headers};
use http::HeaderMap;

// Inject trace context into outgoing requests
let mut headers = HeaderMap::new();
inject_headers(&mut headers);

// Extract trace context from incoming requests
let parent_context = extract_headers(&headers);
let span = info_span!("child_operation");
let _guard = span.enter();
```

## Configuration Options

### TelemetryConfig

```rust
let config = TelemetryConfig {
    // Required: your service name
    service_name: "my-service".into(),
    
    // Required: service version for tracking
    service_version: env!("CARGO_PKG_VERSION").into(),
    
    // Required: deployment environment
    deployment_env: "production".into(),
    
    // Required: log level (trace, debug, info, warn, error)
    log_level: "info".into(),
};
```

## Best Practices

1. **Initialization:**
   - Use `Telemetry::init()` for standard setups
   - Use modular initialization when you need custom configuration
   - Always call `shutdown()` during cleanup

2. **Span Creation:**
   - Use `tracing` macros for automatic context propagation
   - Keep spans focused and well-named
   - Ensure proper span cleanup

3. **Error Handling:**
   - Handle initialization errors appropriately
   - Validate configuration before use
   - Use error propagation for debugging

## Testing

Run the test suite:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust)
- [AWS X-Ray](https://aws.amazon.com/xray/)
- [tracing](https://github.com/tokio-rs/tracing)

## Support

For issues and feature requests, please use the [GitHub issue tracker](https://github.com/cosmicmind/opentelemetry-xray/issues).