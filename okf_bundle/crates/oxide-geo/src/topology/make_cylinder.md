---
okf_version: "0.2"
type: Function
title: make_cylinder
description: "Create a polygonal B-Rep cylinder with radius, height, and segment count."
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/make_cylinder
language: rust
---

# make_cylinder

Create a polygonal B-Rep cylinder with radius, height, and segment count.

## Signature

```rust
impl TopologyDatabase { pub fn make_cylinder(&mut self, radius: f64, height: f64, segments: usize) -> SolidKey }
```

## Visibility

- `pub`

## Docstring

Create a polygonal B-Rep cylinder with radius, height, and segment count.

## Source
Lines 223–286 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
