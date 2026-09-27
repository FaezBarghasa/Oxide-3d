---
okf_version: "0.2"
type: Function
title: cone
description: Generate a 3D cone centered at origin along the Z axis.
resource: crates/oxide-render/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:09:07Z"
concept_id: crates/oxide-render/src/mesh/cone_1
language: rust
---

# cone

Generate a 3D cone centered at origin along the Z axis.

## Signature

```rust
pub fn cone(radius: f32, height: f32, segments: u32, color: [f32; 4]) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Generate a 3D cone centered at origin along the Z axis.
[must_use]

## Source
Lines 238–280 in `crates/oxide-render/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/oxide-render/src/mesh.md) |
