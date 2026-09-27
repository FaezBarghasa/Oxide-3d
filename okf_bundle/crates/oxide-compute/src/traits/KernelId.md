---
okf_version: "0.2"
type: Class
title: KernelId
description: Handle to a compiled compute kernel / pipeline.
resource: crates/oxide-compute/src/traits.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/traits/KernelId
language: rust
---

# KernelId

Handle to a compiled compute kernel / pipeline.

## Signature

```rust
pub struct KernelId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Handle to a compiled compute kernel / pipeline.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Source
Lines 114–114 in `crates/oxide-compute/src/traits.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [traits](/crates/oxide-compute/src/traits.md) |
| called_by | [compile_kernel](/crates/oxide-compute/src/cpu/compile_kernel.md) |
| called_by | [compile_kernel](/crates/oxide-compute/src/native/compile_kernel.md) |
