---
okf_version: "0.2"
type: Function
title: add_face
description: Add a bounded face.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/add_face
language: rust
---

# add_face

Add a bounded face.

## Signature

```rust
impl TopologyDatabase { pub fn add_face(&mut self, outer_wire: WireKey, surface: Surface3d) -> FaceKey }
```

## Visibility

- `pub`

## Docstring

Add a bounded face.

## Source
Lines 166–168 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
