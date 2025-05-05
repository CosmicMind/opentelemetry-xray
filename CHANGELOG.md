# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2024-05-29

### Fixed
- Fixed an issue where exception attributes (using the `exception.` prefix) were not correctly appearing in the exported X-Ray segment data.

## [0.1.3] - 2024-05-28

### Added
- Initial public release of the opentelemetry-xray crate.
- Support for AWS X-Ray distributed tracing with OpenTelemetry.
- Conversion of OpenTelemetry trace context to AWS X-Ray format.
- Support for annotations, metadata, exceptions, and HTTP context in spans.
