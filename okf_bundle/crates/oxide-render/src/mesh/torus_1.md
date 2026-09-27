---
okf_version: "0.2"
type: Function
title: torus
description: Generate a 3D torus centered at origin.
resource: crates/oxide-render/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:09:07Z"
concept_id: crates/oxide-render/src/mesh/torus_1
language: rust
---

# torus

Generate a 3D torus centered at origin.

## Signature

```rust
pub fn torus(
        major_radius: f32,
        minor_radius: f32,
        major_segments: u32,
        minor_segments: u32,
        color: [f32; 4],
    ) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Generate a 3D torus centered at origin.
[must_use]

## Source
Lines 284–332 in `crates/oxide-render/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/oxide-render/src/mesh.md) |
