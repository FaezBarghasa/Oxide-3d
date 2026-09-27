---
okf_version: "0.2"
type: Function
title: grid
description: Generate a 2D planar grid (XY plane).
resource: crates/oxide-render/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:09:07Z"
concept_id: crates/oxide-render/src/mesh/grid
language: rust
---

# grid

Generate a 2D planar grid (XY plane).

## Signature

```rust
impl TriMesh { pub fn grid(size_x: f32, size_y: f32, subdiv_x: u32, subdiv_y: u32, color: [f32; 4]) -> Self }
```

## Visibility

- `pub`

## Docstring

Generate a 2D planar grid (XY plane).
[must_use]

## Source
Lines 336–370 in `crates/oxide-render/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/oxide-render/src/mesh.md) |
