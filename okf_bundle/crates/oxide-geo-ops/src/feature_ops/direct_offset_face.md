---
okf_version: "0.2"
type: Function
title: direct_offset_face
description: Apply a direct modeling offset to all vertices of a planar face along its normal.
resource: crates/oxide-geo-ops/src/feature_ops.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-ops/src/feature_ops/direct_offset_face
language: rust
---

# direct_offset_face

Apply a direct modeling offset to all vertices of a planar face along its normal.

## Signature

```rust
pub fn direct_offset_face(
    db: &mut TopologyDatabase,
    face_key: FaceKey,
    offset_distance: f64,
) -> bool
```

## Visibility

- `pub`

## Docstring

Apply a direct modeling offset to all vertices of a planar face along its normal.

## Source
Lines 277–333 in `crates/oxide-geo-ops/src/feature_ops.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [feature_ops](/crates/oxide-geo-ops/src/feature_ops.md) |
| called_by | [test_primitive_constructors_and_direct_offset](/crates/oxide-geo-ops/src/feature_ops/test_primitive_constructors_and_direct_offset.md) |
