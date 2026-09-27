---
okf_version: "0.2"
type: Function
title: to_nalgebra_isometry
description: Convert to nalgebra Isometry3 for physics and kinematics.
resource: crates/oxide-math/src/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-math/src/transform/to_nalgebra_isometry
language: rust
---

# to_nalgebra_isometry

Convert to nalgebra Isometry3 for physics and kinematics.

## Signature

```rust
impl Transform3 { pub fn to_nalgebra_isometry(&self) -> Isometry3<f64> }
```

## Visibility

- `pub`

## Docstring

Convert to nalgebra Isometry3 for physics and kinematics.
[must_use]

## Source
Lines 42–55 in `crates/oxide-math/src/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-math/src/transform.md) |
