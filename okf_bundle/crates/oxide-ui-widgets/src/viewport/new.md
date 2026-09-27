---
okf_version: "0.2"
type: Function
title: new
description: Create a new interactive viewport widget displaying a camera and mesh.
resource: crates/oxide-ui-widgets/src/viewport.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ui-widgets"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-ui-widgets/src/viewport/new
language: rust
---

# new

Create a new interactive viewport widget displaying a camera and mesh.

## Signature

```rust
impl ViewportWidget<'a> { pub fn new(camera: &'a Camera, mesh: &'a TriMesh) -> Self }
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Create a new interactive viewport widget displaying a camera and mesh.
[must_use]

## Source
Lines 64–66 in `crates/oxide-ui-widgets/src/viewport.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [viewport](/crates/oxide-ui-widgets/src/viewport.md) |
