---
okf_version: "0.2"
type: Function
title: dispatch
resource: crates/oxide-compute/src/native.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/native/dispatch
language: rust
---

# dispatch

## Signature

```rust
impl NativeComputeDevice { fn dispatch(
        &self,
        kernel: KernelId,
        grid_size: [u32; 3],
        workgroup_size: [u32; 3],
        bindings: &[BufferId],
    ) -> Result<(), ComputeError> }
```

## Source
Lines 159–209 in `crates/oxide-compute/src/native.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [native](/crates/oxide-compute/src/native.md) |
