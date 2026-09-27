---
okf_version: "0.2"
type: Function
title: launch
resource: crates/oxide-backend-rocm/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-rocm"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:05:07Z"
concept_id: crates/oxide-backend-rocm/src/lib/launch_1
language: rust
---

# launch

## Signature

```rust
fn launch(
        &self,
        _kernel: &KernelHandle,
        _grid: GridDim,
        _block: BlockDim,
        _args: &[KernelArg],
    ) -> Result<KernelEvent, ComputeError>
```

## Source
Lines 70–81 in `crates/oxide-backend-rocm/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-rocm/src/lib.md) |
