---
okf_version: "0.2"
type: Function
title: retract
description: Retract tangent vector xi in se(3) to SE(3) manifold via exponential map.
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/retract_1
language: rust
---

# retract

Retract tangent vector xi in se(3) to SE(3) manifold via exponential map.

## Signature

```rust
pub fn retract(&self, xi: &[f64; 6]) -> Self
```

## Visibility

- `pub`

## Docstring

Retract tangent vector xi in se(3) to SE(3) manifold via exponential map.

## Source
Lines 121–138 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
