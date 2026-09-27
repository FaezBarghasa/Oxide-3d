---
okf_version: "0.2"
type: Function
title: from_axis_angle
description: "Construct from rotation axis [ax, ay, az] and angle in radians."
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/from_axis_angle
language: rust
---

# from_axis_angle

Construct from rotation axis [ax, ay, az] and angle in radians.

## Signature

```rust
impl DualQuaternion { pub fn from_axis_angle(axis: [f64; 3], angle_rad: f64) -> Self }
```

## Visibility

- `pub`

## Docstring

Construct from rotation axis [ax, ay, az] and angle in radians.

## Source
Lines 51–62 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
