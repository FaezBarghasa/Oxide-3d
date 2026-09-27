---
okf_version: "0.2"
type: Function
title: extrude_face
description: Extrude a planar B-Rep face along a vector into a closed 3D solid body.
resource: crates/oxide-geo-ops/src/feature_ops.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-ops/src/feature_ops/extrude_face
language: rust
---

# extrude_face

Extrude a planar B-Rep face along a vector into a closed 3D solid body.

## Signature

```rust
pub fn extrude_face(
    db: &mut TopologyDatabase,
    face_key: FaceKey,
    direction: [f64; 3],
    opts: ExtrudeOptions,
) -> Option<SolidKey>
```

## Visibility

- `pub`

## Docstring

Extrude a planar B-Rep face along a vector into a closed 3D solid body.

## Source
Lines 64–166 in `crates/oxide-geo-ops/src/feature_ops.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [feature_ops](/crates/oxide-geo-ops/src/feature_ops.md) |
| called_by | [test_extrude_and_revolve_feature_ops](/crates/oxide-geo-ops/src/feature_ops/test_extrude_and_revolve_feature_ops.md) |
| called_by | [extrude](/crates/oxide-geo-ops/src/kernel/extrude.md) |
