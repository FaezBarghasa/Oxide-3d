---
okf_version: "0.2"
type: Function
title: download_bytes
resource: crates/oxide-backend-cpu/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-cpu"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-backend-cpu/src/lib/download_bytes_1
language: rust
---

# download_bytes

## Signature

```rust
fn download_bytes(&self, buffer: &DeviceBuffer, out: &mut [u8]) -> Result<(), ComputeError>
```

## Source
Lines 88–100 in `crates/oxide-backend-cpu/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-cpu/src/lib.md) |
