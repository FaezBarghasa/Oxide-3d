---
okf_version: "0.2"
type: Function
title: dispatch
resource: crates/oxide-compute/src/cpu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/cpu/dispatch_1
language: rust
---

# dispatch

## Signature

```rust
fn dispatch(
        &self,
        _kernel: KernelId,
        _grid_size: [u32; 3],
        _workgroup_size: [u32; 3],
        _bindings: &[BufferId],
    ) -> Result<(), ComputeError>
```

## Source
Lines 134–143 in `crates/oxide-compute/src/cpu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cpu](/crates/oxide-compute/src/cpu.md) |
