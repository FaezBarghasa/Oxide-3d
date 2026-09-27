---
okf_version: "0.2"
type: Function
title: project_point_3d_to_2d
description: "Transform 3D coordinate [x, y, z] to 2D sheet viewport coordinate [u, v]."
resource: crates/oxide-geo-ops/src/drawing_sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:39:35Z"
concept_id: crates/oxide-geo-ops/src/drawing_sheet/project_point_3d_to_2d
language: rust
---

# project_point_3d_to_2d

Transform 3D coordinate [x, y, z] to 2D sheet viewport coordinate [u, v].

## Signature

```rust
fn project_point_3d_to_2d(
    p: [f64; 3],
    kind: DrawingViewKind,
    center_mm: [f64; 2],
    scale: f64,
) -> [f64; 2]
```

## Docstring

Transform 3D coordinate [x, y, z] to 2D sheet viewport coordinate [u, v].

## Source
Lines 214–234 in `crates/oxide-geo-ops/src/drawing_sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawing_sheet](/crates/oxide-geo-ops/src/drawing_sheet.md) |
| called_by | [project_mesh_to_view](/crates/oxide-geo-ops/src/drawing_sheet/project_mesh_to_view.md) |
