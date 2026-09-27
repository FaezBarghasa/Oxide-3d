---
okf_version: "0.2"
type: Function
title: compute_solid_bbox
description: Compute the exact bounding box for a solid by evaluating its boundary vertices.
resource: crates/oxide-geo-ops/src/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/boolean/compute_solid_bbox
language: rust
---

# compute_solid_bbox

Compute the exact bounding box for a solid by evaluating its boundary vertices.

## Signature

```rust
pub fn compute_solid_bbox(db: &TopologyDatabase, solid_key: SolidKey) -> Option<BoundingBox3d>
```

## Visibility

- `pub`

## Docstring

Compute the exact bounding box for a solid by evaluating its boundary vertices.

## Source
Lines 79–112 in `crates/oxide-geo-ops/src/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-geo-ops/src/boolean.md) |
| called_by | [boolean_op](/crates/oxide-geo-ops/src/boolean/boolean_op.md) |
| called_by | [test_boolean_intersection_and_union](/crates/oxide-geo-ops/src/boolean/test_boolean_intersection_and_union.md) |
