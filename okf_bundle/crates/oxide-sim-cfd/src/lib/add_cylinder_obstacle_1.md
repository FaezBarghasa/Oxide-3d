---
okf_version: "0.2"
type: Function
title: add_cylinder_obstacle
description: Add a circular solid obstruction (e.g. cylinder in crossflow).
resource: crates/oxide-sim-cfd/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim-cfd"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-10T10:07:18Z"
concept_id: crates/oxide-sim-cfd/src/lib/add_cylinder_obstacle_1
language: rust
---

# add_cylinder_obstacle

Add a circular solid obstruction (e.g. cylinder in crossflow).

## Signature

```rust
pub fn add_cylinder_obstacle(&mut self, cx: f64, cy: f64, radius: f64)
```

## Visibility

- `pub`

## Docstring

Add a circular solid obstruction (e.g. cylinder in crossflow).

## Source
Lines 78–90 in `crates/oxide-sim-cfd/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-sim-cfd/src/lib.md) |
