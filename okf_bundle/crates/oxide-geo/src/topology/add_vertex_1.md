---
okf_version: "0.2"
type: Function
title: add_vertex
description: Add a 3D vertex to the database.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/add_vertex_1
language: rust
---

# add_vertex

Add a 3D vertex to the database.

## Signature

```rust
pub fn add_vertex(&mut self, point: [f64; 3]) -> VertexKey
```

## Visibility

- `pub`

## Docstring

Add a 3D vertex to the database.

## Source
Lines 146–148 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
