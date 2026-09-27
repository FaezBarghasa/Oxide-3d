---
okf_version: "0.2"
type: Function
title: download_bytes
resource: crates/oxide-backend-vulkan/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-vulkan"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:05:28Z"
concept_id: crates/oxide-backend-vulkan/src/lib/download_bytes
language: rust
---

# download_bytes

## Signature

```rust
impl VulkanBackend { fn download_bytes(&self, _buffer: &DeviceBuffer, _out: &mut [u8]) -> Result<(), ComputeError> }
```

## Source
Lines 64–66 in `crates/oxide-backend-vulkan/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-vulkan/src/lib.md) |
