# FemtoClaw Audit

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Normative-green.svg)]()

FemtoClaw Observability — telemetry, logging, metrics, and audit trail.

## Overview

`femtoclaw-audit` provides the observability and telemetry layer for the FemtoClaw Industrial Agent Runtime. It implements complete runtime visibility according to the [FemtoClaw Observability Specification (FC-08)](../femtoclaw-spec/08-FemtoClaw_Observability_and_Telemetry_Specification.md).

Observability is a mandatory property of FemtoClaw, ensuring that every runtime decision is visible and auditable.

## Features

- **Telemetry Events**: Structured event capture for all runtime activity
- **Structured Logging**: Comprehensive logging with JSON output
- **Metrics**: Counter, Gauge, and Histogram metrics
- **Audit Trail**: Complete execution history for compliance
- **Trace Support**: Distributed tracing integration

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    FemtoClaw Runtime                        │
├─────────────────────────────────────────────────────────────┤
│  Input → Agent Core → Protocol → Policy → Execution → Memory│
│         ↓           ↓         ↓        ↓         ↓         │
│  ┌─────────────────────────────────────────────────────┐  │
│  │              femtoclaw-audit                         │  │
│  │  ┌───────────┐ ┌────────┐ ┌─────────┐ ┌───────────┐   │  │
│  │  │  Events   │ │ Logger │ │ Metrics │ │  Audit   │   │  │
│  │  └───────────┘ └────────┘ └─────────┘ └───────────┘   │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
                    ┌─────────────────┐
                    │ External Systems│
                    │ - Log Aggregator │
                    │ - Metrics Store  │
                    │ - SIEM           │
                    └─────────────────┘
```

## Captured Events

- Input events and user interactions
- Protocol validation decisions
- Authorization decisions (Allow/Deny)
- Capability executions
- Memory writes and state changes
- Errors and failures
- Runtime state transitions

## Installation

```toml
[dependencies]
femtoclaw-audit = "1.0"
```

## Usage

```rust
use femtoclaw_audit::{Event, EventType, Logger, Metrics};

// Create events
let event = Event::new(
    EventType::ProtocolValidation,
    serde_json::json!({"input": "valid", "result": "ok"})
);

// Structured logging
let logger = Logger::new();
logger.log(
    tracing::Level::INFO,
    "femtoclaw",
    "Authorization decision",
    serde_json::json!({"tool": "filesystem.read", "decision": "allow"})
);

// Metrics
let metrics = Metrics::new();
metrics.increment_counter("femtoclaw.capability.executions");
metrics.record_histogram("femtoclaw.execution.time_ms", 42.5);
```

## Modules

- `event` — Telemetry event types and definitions
- `logger` — Structured logging implementation
- `metrics` — Metrics collection (Counter, Gauge, Histogram)

## Requirements

- Rust 1.75 or later
- serde 1.x
- serde_json 1.x
- chrono 0.4 (with serde)
- uuid 1.x (with v4, serde)
- tracing 0.1
- metrics 0.22

## Related Specifications

- [FC-08: Observability and Telemetry](../femtoclaw-spec/08-FemtoClaw_Observability_and_Telemetry_Specification.md)
- [FC-AUDIT-0001: Audit and Compliance Certification](../femtoclaw-spec/FC-AUDIT-0001-Audit_and_Compliance_Certification_Specification.md)

## License

Copyright 2026 FemtoClaw

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
