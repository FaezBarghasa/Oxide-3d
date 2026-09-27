---
okf_version: "0.2"
type: Function
title: add_view
description: Add a projected viewport for a 3D mesh.
resource: crates/oxide-geo-ops/src/drawing_sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo-ops"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-19T05:39:35Z"
concept_id: crates/oxide-geo-ops/src/drawing_sheet/add_view_1
language: rust
---

# add_view

Add a projected viewport for a 3D mesh.

## Signature

```rust
pub fn add_view(&mut self, mesh: &TessellatedMesh, kind: DrawingViewKind, center_mm: [f64; 2], scale: f64)
```

## Visibility

- `pub`

## Docstring

Add a projected viewport for a 3D mesh.

## Source
Lines 77–85 in `crates/oxide-geo-ops/src/drawing_sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drawing_sheet](/crates/oxide-geo-ops/src/drawing_sheet.md) |
| calls | [project_mesh_to_view](/crates/oxide-geo-ops/src/drawing_sheet/project_mesh_to_view.md) |
