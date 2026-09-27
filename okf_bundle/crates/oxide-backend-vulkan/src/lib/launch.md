---
okf_version: "0.2"
type: Function
title: launch
resource: crates/oxide-backend-vulkan/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-backend-vulkan"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:05:28Z"
concept_id: crates/oxide-backend-vulkan/src/lib/launch
language: rust
---

# launch

## Signature

```rust
impl VulkanBackend { fn launch(
        &self,
        _kernel: &KernelHandle,
        _grid: GridDim,
        _block: BlockDim,
        _args: &[KernelArg],
    ) -> Result<KernelEvent, ComputeError> }
```

## Source
Lines 68–79 in `crates/oxide-backend-vulkan/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-backend-vulkan/src/lib.md) |
