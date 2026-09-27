---
okf_version: "0.2"
type: Function
title: upload_bytes
resource: crates/oxide-backend-cpu/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-cpu"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-backend-cpu/src/lib/upload_bytes
language: rust
---

# upload_bytes

## Signature

```rust
impl CpuBackend { fn upload_bytes(&self, data: &[u8]) -> Result<DeviceBuffer, ComputeError> }
```

## Source
Lines 82–86 in `crates/oxide-backend-cpu/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-cpu/src/lib.md) |
