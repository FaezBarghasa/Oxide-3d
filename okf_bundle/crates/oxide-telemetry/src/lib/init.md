---
okf_version: "0.2"
type: Function
title: init
description: Initialize tracing with environment filtering and console output.
resource: crates/oxide-telemetry/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-telemetry"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-telemetry/src/lib/init
language: rust
---

# init

Initialize tracing with environment filtering and console output.

## Signature

```rust
pub fn init() -> Result<(), TelemetryError>
```

## Visibility

- `pub`

## Docstring

Initialize tracing with environment filtering and console output.

## Source
Lines 15–32 in `crates/oxide-telemetry/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-telemetry/src/lib.md) |
| called_by | [main](/apps/oxide-desktop/src/main/main.md) |
| called_by | [main](/apps/oxide-server/src/main/main.md) |
