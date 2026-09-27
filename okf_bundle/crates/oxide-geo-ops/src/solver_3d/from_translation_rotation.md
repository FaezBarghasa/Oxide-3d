---
okf_version: "0.2"
type: Function
title: from_translation_rotation
description: "Construct from translation vector [tx, ty, tz] and unit quaternion [qw, qx, qy, qz]."
resource: crates/oxide-geo-ops/src/solver_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:42:04Z"
concept_id: crates/oxide-geo-ops/src/solver_3d/from_translation_rotation
language: rust
---

# from_translation_rotation

Construct from translation vector [tx, ty, tz] and unit quaternion [qw, qx, qy, qz].

## Signature

```rust
impl DualQuaternion { pub fn from_translation_rotation(t: [f64; 3], q: [f64; 4]) -> Self }
```

## Visibility

- `pub`

## Docstring

Construct from translation vector [tx, ty, tz] and unit quaternion [qw, qx, qy, qz].

## Source
Lines 34–43 in `crates/oxide-geo-ops/src/solver_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_3d](/crates/oxide-geo-ops/src/solver_3d.md) |
