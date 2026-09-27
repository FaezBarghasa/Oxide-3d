---
okf_version: "0.2"
type: Function
title: add_vertex
description: Add a vertex with 3D coordinate.
resource: crates/oxide-geo/src/half_edge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:07:45Z"
concept_id: crates/oxide-geo/src/half_edge/add_vertex
language: rust
---

# add_vertex

Add a vertex with 3D coordinate.

## Signature

```rust
impl HalfEdgeMesh { pub fn add_vertex(&mut self, position: [f64; 3]) -> HeVertexKey }
```

## Visibility

- `pub`

## Docstring

Add a vertex with 3D coordinate.

## Source
Lines 97–102 in `crates/oxide-geo/src/half_edge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [half_edge](/crates/oxide-geo/src/half_edge.md) |
