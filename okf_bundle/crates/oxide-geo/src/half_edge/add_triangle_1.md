---
okf_version: "0.2"
type: Function
title: add_triangle
description: Add a triangle face given 3 ordered vertex keys.
resource: crates/oxide-geo/src/half_edge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:07:45Z"
concept_id: crates/oxide-geo/src/half_edge/add_triangle_1
language: rust
---

# add_triangle

Add a triangle face given 3 ordered vertex keys.

## Signature

```rust
pub fn add_triangle(
        &mut self,
        v0: HeVertexKey,
        v1: HeVertexKey,
        v2: HeVertexKey,
    ) -> Option<HeFaceKey>
```

## Visibility

- `pub`

## Docstring

Add a triangle face given 3 ordered vertex keys.

## Source
Lines 105–177 in `crates/oxide-geo/src/half_edge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [half_edge](/crates/oxide-geo/src/half_edge.md) |
