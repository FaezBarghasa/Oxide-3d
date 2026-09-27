---
okf_version: "0.2"
type: Function
title: update_densities
description: Perform an Optimality Criteria (OC) density update step given element strain energy sensitivities.
resource: crates/oxide-sim-topopt/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim-topopt"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T22:11:55Z"
concept_id: crates/oxide-sim-topopt/src/lib/update_densities_1
language: rust
---

# update_densities

Perform an Optimality Criteria (OC) density update step given element strain energy sensitivities.

## Signature

```rust
pub fn update_densities(&mut self, sensitivities: &[f64])
```

## Visibility

- `pub`

## Docstring

Perform an Optimality Criteria (OC) density update step given element strain energy sensitivities.

## Source
Lines 56–102 in `crates/oxide-sim-topopt/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-topopt/src/lib.md) |
