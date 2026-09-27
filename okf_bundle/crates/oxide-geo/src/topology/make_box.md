---
okf_version: "0.2"
type: Function
title: make_box
description: "Create an exact B-Rep solid box with dimensions (dx, dy, dz) centered at origin."
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/make_box
language: rust
---

# make_box

Create an exact B-Rep solid box with dimensions (dx, dy, dz) centered at origin.

## Signature

```rust
impl TopologyDatabase { pub fn make_box(&mut self, dx: f64, dy: f64, dz: f64) -> SolidKey }
```

## Visibility

- `pub`

## Docstring

Create an exact B-Rep solid box with dimensions (dx, dy, dz) centered at origin.

## Source
Lines 181–220 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
