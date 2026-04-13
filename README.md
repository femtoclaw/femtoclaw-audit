# 📋 FemtoClaw Audit & Telemetry

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/Status-Normative-green.svg)]()

The **FemtoClaw Audit** library provides the mandatory observability layer for industrial agent execution. It captures, buffers, and logs every runtime decision, ensuring that all AI interactions are fully traceable and tamper-evident.

---

## 🛰️ Telemetry Architecture

FemtoClaw observability is designed to be high-performance and asynchronous, preventing logging overhead from introducing jitter into the deterministic control loop.

- **Structured Events**: Every protocol decision, policy evaluation (Allow/Deny), and capability execution is captured as a structured JSON event.
- **Tamper-Evident Logs**: Audit logs are designed for ingestion into industrial SIEMs or forensic analysis tools.
- **Metrics**: Real-time counters and histograms for execution latency, tool usage, and error rates.

---

## 🚀 Usage

```rust
use femtoclaw_audit::{Telemetry, Event};

let telemetry = Telemetry::new();

// Emit a custom capability event
let event = Event::capability_execution_complete("shell", "Success: exit code 0");
telemetry.emit_and_log(event).await;
```

---

## 📊 Event Types

| Event | Description |
|-------|-------------|
| `ProtocolValidation` | Result of JSON schema enforcement. |
| `AuthorizationDecision` | Outcome of the Policy Engine (Allow/Deny). |
| `CapabilityExecution` | Results and metadata from a system "Claw". |
| `StateMutation` | Recorded changes to agent memory or WAL. |

---

## 📄 Related Specifications
- **[FC-08: Observability and Telemetry](../femtoclaw-spec/08-FemtoClaw_Observability_and_Telemetry_Specification.md)**
- **[FC-AUDIT-0001: Audit and Compliance](../femtoclaw-spec/FC-AUDIT-0001-Audit_and_Compliance_Certification_Specification.md)**

Copyright © 2026 FemtoClaw Project.
