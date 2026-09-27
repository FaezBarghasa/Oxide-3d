---
okf_version: "0.2"
type: Function
title: create_primitive_box
description: Create a parametric 3D box solid primitive in the topology database.
resource: crates/oxide-geo-ops/src/feature_ops.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-ops/src/feature_ops/create_primitive_box
language: rust
---

# create_primitive_box

Create a parametric 3D box solid primitive in the topology database.

## Signature

```rust
pub fn create_primitive_box(db: &mut TopologyDatabase, dx: f64, dy: f64, dz: f64) -> SolidKey
```

## Visibility

- `pub`

## Docstring

Create a parametric 3D box solid primitive in the topology database.

## Source
Lines 253–255 in `crates/oxide-geo-ops/src/feature_ops.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [feature_ops](/crates/oxide-geo-ops/src/feature_ops.md) |
| called_by | [test_primitive_constructors_and_direct_offset](/crates/oxide-geo-ops/src/feature_ops/test_primitive_constructors_and_direct_offset.md) |
