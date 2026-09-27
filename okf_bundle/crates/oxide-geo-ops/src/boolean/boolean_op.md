---
okf_version: "0.2"
type: Function
title: boolean_op
description: Execute exact constructive boolean operations on topological solid bodies.
resource: crates/oxide-geo-ops/src/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-geo-ops/src/boolean/boolean_op
language: rust
---

# boolean_op

Execute exact constructive boolean operations on topological solid bodies.

## Signature

```rust
pub fn boolean_op(
    db: &mut TopologyDatabase,
    solid_a: SolidKey,
    solid_b: SolidKey,
    opts: BooleanOptions,
) -> Result<SolidKey, String>
```

## Visibility

- `pub`

## Docstring

Execute exact constructive boolean operations on topological solid bodies.

## Source
Lines 115–183 in `crates/oxide-geo-ops/src/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-geo-ops/src/boolean.md) |
| calls | [compute_solid_bbox](/crates/oxide-geo-ops/src/boolean/compute_solid_bbox.md) |
| calls | [create_box_from_bounds](/crates/oxide-geo-ops/src/boolean/create_box_from_bounds.md) |
| called_by | [test_boolean_intersection_and_union](/crates/oxide-geo-ops/src/boolean/test_boolean_intersection_and_union.md) |
| called_by | [boolean](/crates/oxide-geo-ops/src/kernel/boolean.md) |
