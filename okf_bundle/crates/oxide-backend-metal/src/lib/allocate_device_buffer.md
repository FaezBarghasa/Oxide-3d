---
okf_version: "0.2"
type: Function
title: allocate_device_buffer
resource: crates/oxide-backend-metal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-metal"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:05:53Z"
concept_id: crates/oxide-backend-metal/src/lib/allocate_device_buffer
language: rust
---

# allocate_device_buffer

## Signature

```rust
impl MetalBackend { fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError> }
```

## Source
Lines 52–58 in `crates/oxide-backend-metal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-metal/src/lib.md) |
