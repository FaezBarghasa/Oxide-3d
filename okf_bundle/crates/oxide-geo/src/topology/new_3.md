---
okf_version: "0.2"
type: Function
title: new
description: Create a new edge connecting start and end vertices.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/new_3
language: rust
---

# new

Create a new edge connecting start and end vertices.

## Signature

```rust
pub fn new(start: VertexKey, end: VertexKey, curve: Option<Curve3d>) -> Self
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Create a new edge connecting start and end vertices.
[must_use]

## Source
Lines 39–41 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
