---
okf_version: "0.2"
type: Function
title: new
description: Create a new wire from an ordered list of edge keys.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/new_4
language: rust
---

# new

Create a new wire from an ordered list of edge keys.

## Signature

```rust
impl Wire { pub fn new(edges: impl IntoIterator<Item = EdgeKey>) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new wire from an ordered list of edge keys.
[must_use]

## Source
Lines 54–58 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
