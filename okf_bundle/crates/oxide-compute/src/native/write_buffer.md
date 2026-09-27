---
okf_version: "0.2"
type: Function
title: write_buffer
resource: crates/oxide-compute/src/native.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/native/write_buffer
language: rust
---

# write_buffer

## Signature

```rust
impl NativeComputeDevice { fn write_buffer(
        &self,
        buffer: BufferId,
        offset_bytes: usize,
        data: &[u8],
    ) -> Result<(), ComputeError> }
```

## Source
Lines 97–121 in `crates/oxide-compute/src/native.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [native](/crates/oxide-compute/src/native.md) |
