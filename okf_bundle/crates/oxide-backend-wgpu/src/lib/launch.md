---
okf_version: "0.2"
type: Function
title: launch
resource: crates/oxide-backend-wgpu/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-wgpu"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:04:25Z"
concept_id: crates/oxide-backend-wgpu/src/lib/launch
language: rust
---

# launch

## Signature

```rust
impl WgpuBackend { fn launch(
        &self,
        _kernel: &KernelHandle,
        _grid: GridDim,
        _block: BlockDim,
        _args: &[KernelArg],
    ) -> Result<KernelEvent, ComputeError> }
```

## Source
Lines 68–79 in `crates/oxide-backend-wgpu/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-wgpu/src/lib.md) |
