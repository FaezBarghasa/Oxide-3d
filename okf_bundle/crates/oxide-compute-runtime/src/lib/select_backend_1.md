---
okf_version: "0.2"
type: Function
title: select_backend
description: Select optimal backend satisfying the kernel requirements.
resource: crates/oxide-compute-runtime/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute-runtime"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute-runtime/src/lib/select_backend_1
language: rust
---

# select_backend

Select optimal backend satisfying the kernel requirements.

## Signature

```rust
pub fn select_backend(
        &self,
        req: &KernelRequirement,
    ) -> Result<Arc<dyn AcceleratorBackend>, ComputeError>
```

## Visibility

- `pub`

## Docstring

Select optimal backend satisfying the kernel requirements.

## Source
Lines 82–94 in `crates/oxide-compute-runtime/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-compute-runtime/src/lib.md) |
