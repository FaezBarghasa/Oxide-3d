---
okf_version: "0.2"
type: Function
title: upload_bytes
resource: crates/oxide-backend-cuda/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-cuda"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:04:46Z"
concept_id: crates/oxide-backend-cuda/src/lib/upload_bytes_1
language: rust
---

# upload_bytes

## Signature

```rust
fn upload_bytes(&self, data: &[u8]) -> Result<DeviceBuffer, ComputeError>
```

## Source
Lines 62–64 in `crates/oxide-backend-cuda/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-cuda/src/lib.md) |
