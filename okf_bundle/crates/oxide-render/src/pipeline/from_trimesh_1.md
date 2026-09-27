---
okf_version: "0.2"
type: Function
title: from_trimesh
description: Upload CPU TriMesh to GPU buffers.
resource: crates/oxide-render/src/pipeline.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-08T21:42:20Z"
concept_id: crates/oxide-render/src/pipeline/from_trimesh_1
language: rust
---

# from_trimesh

Upload CPU TriMesh to GPU buffers.

## Signature

```rust
pub fn from_trimesh(device: &wgpu::Device, mesh: &TriMesh) -> Self
```

## Visibility

- `pub`

## Docstring

Upload CPU TriMesh to GPU buffers.

## Source
Lines 41–59 in `crates/oxide-render/src/pipeline.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pipeline](/crates/oxide-render/src/pipeline.md) |
