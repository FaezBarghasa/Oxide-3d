---
okf_version: "0.2"
type: Class
title: TopOptConfig
description: Configuration parameters for SIMP Topology Optimization.
resource: crates/oxide-sim-topopt/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim-topopt"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T22:11:55Z"
concept_id: crates/oxide-sim-topopt/src/lib/TopOptConfig
language: rust
---

# TopOptConfig

Configuration parameters for SIMP Topology Optimization.

## Signature

```rust
pub struct TopOptConfig
```

## Decorators

- `derive(Debug, Clone, Copy, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Configuration parameters for SIMP Topology Optimization.
[derive(Debug, Clone, Copy, Serialize, Deserialize)]

## Methods

- `target_volume_fraction`
- `penalization_power`
- `filter_radius`
- `max_iterations`
- `move_limit`

## Source
Lines 7–18 in `crates/oxide-sim-topopt/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-topopt/src/lib.md) |
