---
okf_version: "0.2"
type: Class
title: DeviceCapabilities
description: Hardware and capability profile of a compute device.
resource: crates/oxide-compute/src/traits.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-compute/src/traits/DeviceCapabilities
language: rust
---

# DeviceCapabilities

Hardware and capability profile of a compute device.

## Signature

```rust
pub struct DeviceCapabilities
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Hardware and capability profile of a compute device.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `name`
- `backend`
- `total_memory_bytes`
- `is_unified_memory`
- `supports_fp64`
- `supports_fp16`
- `supports_tensor_cores`
- `max_workgroups`
- `max_workgroup_size`

## Source
Lines 74–93 in `crates/oxide-compute/src/traits.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [traits](/crates/oxide-compute/src/traits.md) |
