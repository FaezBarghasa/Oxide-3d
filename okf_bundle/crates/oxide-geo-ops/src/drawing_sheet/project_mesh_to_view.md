---
okf_version: "0.2"
type: Function
title: project_mesh_to_view
description: Project 3D mesh triangle boundary edges onto a 2D sheet view.
resource: crates/oxide-geo-ops/src/drawing_sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:39:35Z"
concept_id: crates/oxide-geo-ops/src/drawing_sheet/project_mesh_to_view
language: rust
---

# project_mesh_to_view

Project 3D mesh triangle boundary edges onto a 2D sheet view.

## Signature

```rust
fn project_mesh_to_view(
    mesh: &TessellatedMesh,
    kind: DrawingViewKind,
    center_mm: [f64; 2],
    scale: f64,
) -> Vec<ProjectedEdge2D>
```

## Docstring

Project 3D mesh triangle boundary edges onto a 2D sheet view.

## Source
Lines 179–211 in `crates/oxide-geo-ops/src/drawing_sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawing_sheet](/crates/oxide-geo-ops/src/drawing_sheet.md) |
| calls | [project_point_3d_to_2d](/crates/oxide-geo-ops/src/drawing_sheet/project_point_3d_to_2d.md) |
| called_by | [add_view](/crates/oxide-geo-ops/src/drawing_sheet/add_view.md) |
