---
okf_version: "0.2"
type: Function
title: upload_bytes
resource: crates/oxide-backend-dx12/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-dx12"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:06:10Z"
concept_id: crates/oxide-backend-dx12/src/lib/upload_bytes
language: rust
---

# upload_bytes

## Signature

```rust
impl Dx12Backend { fn upload_bytes(&self, data: &[u8]) -> Result<DeviceBuffer, ComputeError> }
```

## Source
Lines 60–62 in `crates/oxide-backend-dx12/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-dx12/src/lib.md) |
