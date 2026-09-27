---
okf_version: "0.2"
type: Function
title: add_wire
description: Add a closed boundary wire.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/add_wire
language: rust
---

# add_wire

Add a closed boundary wire.

## Signature

```rust
impl TopologyDatabase { pub fn add_wire(&mut self, edges: impl IntoIterator<Item = EdgeKey>) -> WireKey }
```

## Visibility

- `pub`

## Docstring

Add a closed boundary wire.

## Source
Lines 161–163 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
