---
okf_version: "0.2"
type: Function
title: compute_residuals
description: Evaluate scalar residual vector across all constraints.
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/compute_residuals
language: rust
---

# compute_residuals

Evaluate scalar residual vector across all constraints.

## Signature

```rust
impl ConstraintSolver3D { pub fn compute_residuals(&self) -> Vec<f64> }
```

## Visibility

- `pub`

## Docstring

Evaluate scalar residual vector across all constraints.

## Source
Lines 364–434 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
