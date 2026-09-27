---
okf_version: "0.2"
type: Function
title: upload_bytes
resource: crates/oxide-backend-rocm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-rocm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:05:07Z"
concept_id: crates/oxide-backend-rocm/src/lib/upload_bytes_1
language: rust
---

# upload_bytes

## Signature

```rust
fn upload_bytes(&self, data: &[u8]) -> Result<DeviceBuffer, ComputeError>
```

## Source
Lines 62–64 in `crates/oxide-backend-rocm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-rocm/src/lib.md) |
