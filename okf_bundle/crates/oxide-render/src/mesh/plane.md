---
okf_version: "0.2"
type: Function
title: plane
description: Generate a 3D plane (quad) centered at origin facing +Z.
resource: crates/oxide-render/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:09:07Z"
concept_id: crates/oxide-render/src/mesh/plane
language: rust
---

# plane

Generate a 3D plane (quad) centered at origin facing +Z.

## Signature

```rust
impl TriMesh { pub fn plane(size: f32, color: [f32; 4]) -> Self }
```

## Visibility

- `pub`

## Docstring

Generate a 3D plane (quad) centered at origin facing +Z.
[must_use]

## Source
Lines 224–234 in `crates/oxide-render/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/oxide-render/src/mesh.md) |
