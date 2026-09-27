---
okf_version: "0.2"
type: Function
title: new
description: "Create a new vertex with position, normal, and color."
resource: crates/oxide-render/src/mesh.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-render"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:09:07Z"
concept_id: crates/oxide-render/src/mesh/new_1
language: rust
---

# new

Create a new vertex with position, normal, and color.

## Signature

```rust
pub fn new(position: [f32; 3], normal: [f32; 3], color: [f32; 4]) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create a new vertex with position, normal, and color.
[must_use]

## Source
Lines 21–27 in `crates/oxide-render/src/mesh.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh](/crates/oxide-render/src/mesh.md) |
