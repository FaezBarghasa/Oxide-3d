---
okf_version: "0.2"
type: Module
title: half_edge
description: "Manifold Half-Edge Data Structure for mesh editing, subdivisions, and sculpting."
resource: crates/oxide-geo/src/half_edge.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:07:45Z"
concept_id: crates/oxide-geo/src/half_edge
language: rust
---

# half_edge

Manifold Half-Edge Data Structure for mesh editing, subdivisions, and sculpting.

## Docstring

Manifold Half-Edge Data Structure for mesh editing, subdivisions, and sculpting.

Implements Euler operators (MEV, KEV, MEF, KEF, SEMV) for topological
manipulation while maintaining the Euler-Poincaré invariant:
V - E + F = 2(S - G) + B
where V=vertices, E=edges, F=faces, S=shells, G=genus, B=boundary loops.

## Relationships

| Type | Target |
|------|--------|
| related | [EulerError](/crates/oxide-geo/src/half_edge/EulerError.md) |
| related | [HeVertex](/crates/oxide-geo/src/half_edge/HeVertex.md) |
| related | [HalfEdge](/crates/oxide-geo/src/half_edge/HalfEdge.md) |
| related | [HeFace](/crates/oxide-geo/src/half_edge/HeFace.md) |
| related | [HalfEdgeMesh](/crates/oxide-geo/src/half_edge/HalfEdgeMesh.md) |
| related | [new](/crates/oxide-geo/src/half_edge/new.md) |
| related | [add_vertex](/crates/oxide-geo/src/half_edge/add_vertex.md) |
| related | [add_triangle](/crates/oxide-geo/src/half_edge/add_triangle.md) |
| related | [laplacian_smooth](/crates/oxide-geo/src/half_edge/laplacian_smooth.md) |
| related | [new](/crates/oxide-geo/src/half_edge/new.md) |
| related | [add_vertex](/crates/oxide-geo/src/half_edge/add_vertex.md) |
| related | [add_triangle](/crates/oxide-geo/src/half_edge/add_triangle.md) |
| related | [laplacian_smooth](/crates/oxide-geo/src/half_edge/laplacian_smooth.md) |
| related | [glam](/_dependencies/cargo/glam.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [slotmap](/_dependencies/cargo/slotmap.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
