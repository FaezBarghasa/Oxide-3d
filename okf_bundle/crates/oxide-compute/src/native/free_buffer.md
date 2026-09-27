---
okf_version: "0.2"
type: Function
title: free_buffer
resource: crates/oxide-compute/src/native.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/native/free_buffer
language: rust
---

# free_buffer

## Signature

```rust
impl NativeComputeDevice { fn free_buffer(&self, buffer: BufferId) -> Result<(), ComputeError> }
```

## Source
Lines 82–95 in `crates/oxide-compute/src/native.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [native](/crates/oxide-compute/src/native.md) |
