---
okf_version: "0.2"
type: Function
title: launch
resource: crates/oxide-backend-cpu/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-cpu"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-backend-cpu/src/lib/launch
language: rust
---

# launch

## Signature

```rust
impl CpuBackend { fn launch(
        &self,
        kernel: &KernelHandle,
        _grid: GridDim,
        _block: BlockDim,
        _args: &[KernelArg],
    ) -> Result<KernelEvent, ComputeError> }
```

## Source
Lines 102–115 in `crates/oxide-backend-cpu/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-cpu/src/lib.md) |
