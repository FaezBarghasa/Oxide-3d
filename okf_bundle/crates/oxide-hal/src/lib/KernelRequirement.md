---
okf_version: "0.2"
type: Class
title: KernelRequirement
description: Minimum hardware requirements for a specific simulation or geometric kernel.
resource: crates/oxide-hal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-hal"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:03:44Z"
concept_id: crates/oxide-hal/src/lib/KernelRequirement
language: rust
---

# KernelRequirement

Minimum hardware requirements for a specific simulation or geometric kernel.

## Signature

```rust
pub struct KernelRequirement
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Minimum hardware requirements for a specific simulation or geometric kernel.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `fp64_required`
- `min_memory_bytes`
- `deterministic_required`
- `preferred_backend`

## Source
Lines 109–118 in `crates/oxide-hal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-hal/src/lib.md) |
