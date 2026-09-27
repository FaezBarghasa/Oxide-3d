---
okf_version: "0.2"
type: Function
title: launch
resource: crates/oxide-backend-cuda/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-cuda"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:04:46Z"
concept_id: crates/oxide-backend-cuda/src/lib/launch
language: rust
---

# launch

## Signature

```rust
impl CudaBackend { fn launch(
        &self,
        _kernel: &KernelHandle,
        _grid: GridDim,
        _block: BlockDim,
        _args: &[KernelArg],
    ) -> Result<KernelEvent, ComputeError> }
```

## Source
Lines 70–81 in `crates/oxide-backend-cuda/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-cuda/src/lib.md) |
