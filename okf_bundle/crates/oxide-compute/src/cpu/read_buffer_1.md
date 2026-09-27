---
okf_version: "0.2"
type: Function
title: read_buffer
resource: crates/oxide-compute/src/cpu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/cpu/read_buffer_1
language: rust
---

# read_buffer

## Signature

```rust
fn read_buffer(
        &self,
        buffer: BufferId,
        offset_bytes: usize,
        out: &mut [u8],
    ) -> Result<(), ComputeError>
```

## Source
Lines 107–127 in `crates/oxide-compute/src/cpu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cpu](/crates/oxide-compute/src/cpu.md) |
