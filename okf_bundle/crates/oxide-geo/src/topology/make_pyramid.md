---
okf_version: "0.2"
type: Function
title: make_pyramid
description: Create an exact B-Rep 4-sided pyramid with base size and height.
resource: crates/oxide-geo/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-geo"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:49:43Z"
concept_id: crates/oxide-geo/src/topology/make_pyramid
language: rust
---

# make_pyramid

Create an exact B-Rep 4-sided pyramid with base size and height.

## Signature

```rust
impl TopologyDatabase { pub fn make_pyramid(&mut self, base_size: f64, height: f64) -> SolidKey }
```

## Visibility

- `pub`

## Docstring

Create an exact B-Rep 4-sided pyramid with base size and height.

## Source
Lines 289–331 in `crates/oxide-geo/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-geo/src/topology.md) |
