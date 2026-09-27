---
okf_version: "0.2"
type: Function
title: allocate_buffer
resource: crates/oxide-compute/src/native.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/native/allocate_buffer
language: rust
---

# allocate_buffer

## Signature

```rust
impl NativeComputeDevice { fn allocate_buffer(
        &self,
        size_bytes: usize,
        usage: BufferUsage,
    ) -> Result<BufferId, ComputeError> }
```

## Source
Lines 60–80 in `crates/oxide-compute/src/native.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [native](/crates/oxide-compute/src/native.md) |
| calls | [BufferId](/crates/oxide-compute/src/traits/BufferId.md) |
