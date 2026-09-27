---
okf_version: "0.2"
type: Class
title: BackendCapabilities
description: Comprehensive hardware capability profile of an accelerator.
resource: crates/oxide-hal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-hal"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:03:44Z"
concept_id: crates/oxide-hal/src/lib/BackendCapabilities
language: rust
---

# BackendCapabilities

Comprehensive hardware capability profile of an accelerator.

## Signature

```rust
pub struct BackendCapabilities
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Comprehensive hardware capability profile of an accelerator.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `kind`
- `device_name`
- `fp16`
- `fp32`
- `fp64`
- `int64_atomics`
- `unified_memory`
- `max_workgroup_size`
- `subgroup_size`
- `device_memory_bytes`
- `host_memory_bytes`
- `deterministic_reduction`
- `p2p_multi_gpu`

## Source
Lines 78–105 in `crates/oxide-hal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-hal/src/lib.md) |
