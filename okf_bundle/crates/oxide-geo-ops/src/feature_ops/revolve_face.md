---
okf_version: "0.2"
type: Function
title: revolve_face
description: Revolve a planar face around an axis into a rotational 3D B-Rep solid body.
resource: crates/oxide-geo-ops/src/feature_ops.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:52:49Z"
concept_id: crates/oxide-geo-ops/src/feature_ops/revolve_face
language: rust
---

# revolve_face

Revolve a planar face around an axis into a rotational 3D B-Rep solid body.

## Signature

```rust
pub fn revolve_face(
    db: &mut TopologyDatabase,
    face_key: FaceKey,
    opts: RevolveOptions,
) -> Option<SolidKey>
```

## Visibility

- `pub`

## Docstring

Revolve a planar face around an axis into a rotational 3D B-Rep solid body.

## Source
Lines 169–250 in `crates/oxide-geo-ops/src/feature_ops.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [feature_ops](/crates/oxide-geo-ops/src/feature_ops.md) |
| called_by | [test_extrude_and_revolve_feature_ops](/crates/oxide-geo-ops/src/feature_ops/test_extrude_and_revolve_feature_ops.md) |
