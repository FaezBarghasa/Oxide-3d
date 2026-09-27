---
okf_version: "0.2"
type: Function
title: transform_point
description: Transform a 3D point using this dual quaternion.
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/transform_point
language: rust
---

# transform_point

Transform a 3D point using this dual quaternion.

## Signature

```rust
impl DualQuaternion { pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] }
```

## Visibility

- `pub`

## Docstring

Transform a 3D point using this dual quaternion.

## Source
Lines 84–101 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
