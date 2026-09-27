---
okf_version: "0.2"
type: Function
title: tessellate_solid
description: Tessellate a Solid from the topology database into a triangle mesh.
resource: crates/oxide-geo/src/mesh_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:04:11Z"
concept_id: crates/oxide-geo/src/mesh_bridge/tessellate_solid
language: rust
---

# tessellate_solid

Tessellate a Solid from the topology database into a triangle mesh.

## Signature

```rust
impl BrepTessellator { pub fn tessellate_solid(&self, db: &TopologyDatabase, solid_key: SolidKey) -> TessellatedMesh }
```

## Visibility

- `pub`

## Docstring

Tessellate a Solid from the topology database into a triangle mesh.
[must_use]

## Source
Lines 51–107 in `crates/oxide-geo/src/mesh_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mesh_bridge](/crates/oxide-geo/src/mesh_bridge.md) |
