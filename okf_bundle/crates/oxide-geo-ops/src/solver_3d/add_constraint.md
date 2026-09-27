---
okf_version: "0.2"
type: Function
title: add_constraint
description: Add a 3D geometric constraint.
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/add_constraint
language: rust
---

# add_constraint

Add a 3D geometric constraint.

## Signature

```rust
impl ConstraintSolver3D { pub fn add_constraint(&mut self, constraint: Constraint3D) }
```

## Visibility

- `pub`

## Docstring

Add a 3D geometric constraint.

## Source
Lines 342–344 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
