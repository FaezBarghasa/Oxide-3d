---
okf_version: "0.2"
type: Class
title: HalfEdge
description: Directed half-edge structure.
resource: crates/oxide-geo/src/half_edge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:07:45Z"
concept_id: crates/oxide-geo/src/half_edge/HalfEdge
language: rust
---

# HalfEdge

Directed half-edge structure.

## Signature

```rust
pub struct HalfEdge
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Directed half-edge structure.
[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `twin`
- `next`
- `prev`
- `vertex`
- `face`

## Source
Lines 54–65 in `crates/oxide-geo/src/half_edge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [half_edge](/crates/oxide-geo/src/half_edge.md) |
