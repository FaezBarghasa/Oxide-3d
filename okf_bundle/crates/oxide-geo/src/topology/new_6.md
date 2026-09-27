---
okf_version: "0.2"
type: Function
title: new
description: Create a new face on a parametric surface bounded by an outer wire.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/new_6
language: rust
---

# new

Create a new face on a parametric surface bounded by an outer wire.

## Signature

```rust
impl Face { pub fn new(outer_wire: WireKey, surface: Surface3d) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new face on a parametric surface bounded by an outer wire.
[must_use]

## Source
Lines 75–81 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
