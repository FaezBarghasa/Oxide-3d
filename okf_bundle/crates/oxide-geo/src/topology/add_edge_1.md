---
okf_version: "0.2"
type: Function
title: add_edge
description: Add an edge connecting two vertices.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/add_edge_1
language: rust
---

# add_edge

Add an edge connecting two vertices.

## Signature

```rust
pub fn add_edge(
        &mut self,
        start: VertexKey,
        end: VertexKey,
        curve: Option<Curve3d>,
    ) -> EdgeKey
```

## Visibility

- `pub`

## Docstring

Add an edge connecting two vertices.

## Source
Lines 151–158 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
