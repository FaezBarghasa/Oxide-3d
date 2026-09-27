---
okf_version: "0.2"
type: Function
title: allocate_device_buffer
resource: crates/oxide-backend-cpu/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-cpu"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-backend-cpu/src/lib/allocate_device_buffer_1
language: rust
---

# allocate_device_buffer

## Signature

```rust
fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError>
```

## Source
Lines 71–80 in `crates/oxide-backend-cpu/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-cpu/src/lib.md) |
