---
okf_version: "0.2"
type: Function
title: torus
description: Create a torus mesh.
resource: crates/oxide-nodes/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-nodes"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:10:40Z"
concept_id: crates/oxide-nodes/src/lib/torus
language: rust
---

# torus

Create a torus mesh.

## Signature

```rust
impl NodeMeshData { pub fn torus(
        major_radius: f32,
        minor_radius: f32,
        major_segments: usize,
        minor_segments: usize,
    ) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a torus mesh.

## Source
Lines 252–304 in `crates/oxide-nodes/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-nodes/src/lib.md) |
