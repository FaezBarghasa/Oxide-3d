---
okf_version: "0.2"
type: Function
title: sphere
description: Generate a 3D UV sphere centered at origin.
resource: crates/oxide-render/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:09:07Z"
concept_id: crates/oxide-render/src/mesh/sphere_1
language: rust
---

# sphere

Generate a 3D UV sphere centered at origin.

## Signature

```rust
pub fn sphere(radius: f32, rings: u32, sectors: u32, color: [f32; 4]) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Generate a 3D UV sphere centered at origin.
[must_use]

## Source
Lines 120–161 in `crates/oxide-render/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/oxide-render/src/mesh.md) |
