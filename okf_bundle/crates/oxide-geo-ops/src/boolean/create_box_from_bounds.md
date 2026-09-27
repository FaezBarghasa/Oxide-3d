---
okf_version: "0.2"
type: Function
title: create_box_from_bounds
description: Helper to generate a watertight 6-face solid box from min and max 3D coordinates.
resource: crates/oxide-geo-ops/src/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/boolean/create_box_from_bounds
language: rust
---

# create_box_from_bounds

Helper to generate a watertight 6-face solid box from min and max 3D coordinates.

## Signature

```rust
fn create_box_from_bounds(db: &mut TopologyDatabase, min: [f64; 3], max: [f64; 3]) -> SolidKey
```

## Docstring

Helper to generate a watertight 6-face solid box from min and max 3D coordinates.

## Source
Lines 186–241 in `crates/oxide-geo-ops/src/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-geo-ops/src/boolean.md) |
| called_by | [boolean_op](/crates/oxide-geo-ops/src/boolean/boolean_op.md) |
