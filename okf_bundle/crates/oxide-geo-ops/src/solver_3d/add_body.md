---
okf_version: "0.2"
type: Function
title: add_body
description: Add a body with initial pose and fixed status.
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/add_body
language: rust
---

# add_body

Add a body with initial pose and fixed status.

## Signature

```rust
impl ConstraintSolver3D { pub fn add_body(&mut self, initial_pose: DualQuaternion, fixed: bool) -> usize }
```

## Visibility

- `pub`

## Docstring

Add a body with initial pose and fixed status.

## Source
Lines 334–339 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
