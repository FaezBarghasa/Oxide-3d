---
okf_version: "0.2"
type: Function
title: count_equations
description: Count total scalar residual equations across all constraints.
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/count_equations
language: rust
---

# count_equations

Count total scalar residual equations across all constraints.

## Signature

```rust
impl ConstraintSolver3D { pub fn count_equations(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Count total scalar residual equations across all constraints.

## Source
Lines 347–361 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
