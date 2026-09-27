---
okf_version: "0.2"
type: Function
title: allocate_buffer
resource: crates/oxide-compute/src/cpu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/cpu/allocate_buffer_1
language: rust
---

# allocate_buffer

## Signature

```rust
fn allocate_buffer(
        &self,
        size_bytes: usize,
        _usage: BufferUsage,
    ) -> Result<BufferId, ComputeError>
```

## Source
Lines 61–74 in `crates/oxide-compute/src/cpu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cpu](/crates/oxide-compute/src/cpu.md) |
| calls | [BufferId](/crates/oxide-compute/src/traits/BufferId.md) |
