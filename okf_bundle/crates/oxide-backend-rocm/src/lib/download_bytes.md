---
okf_version: "0.2"
type: Function
title: download_bytes
resource: crates/oxide-backend-rocm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-rocm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:05:07Z"
concept_id: crates/oxide-backend-rocm/src/lib/download_bytes
language: rust
---

# download_bytes

## Signature

```rust
impl RocmBackend { fn download_bytes(&self, _buffer: &DeviceBuffer, _out: &mut [u8]) -> Result<(), ComputeError> }
```

## Source
Lines 66–68 in `crates/oxide-backend-rocm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-rocm/src/lib.md) |
