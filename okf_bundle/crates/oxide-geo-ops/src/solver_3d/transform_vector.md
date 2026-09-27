---
okf_version: "0.2"
type: Function
title: transform_vector
description: Transform a 3D direction vector (rotation only).
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/transform_vector
language: rust
---

# transform_vector

Transform a 3D direction vector (rotation only).

## Signature

```rust
impl DualQuaternion { pub fn transform_vector(&self, v: [f64; 3]) -> [f64; 3] }
```

## Visibility

- `pub`

## Docstring

Transform a 3D direction vector (rotation only).

## Source
Lines 104–118 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
| calls | [px_dummy](/crates/oxide-geo-ops/src/solver_3d/px_dummy.md) |
