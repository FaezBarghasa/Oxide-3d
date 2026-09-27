---
okf_version: "0.2"
type: Function
title: points_equal_2d
description: Check if two 2D points are coincident within linear tolerance.
resource: crates/oxide-math/src/tolerance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:54:05Z"
concept_id: crates/oxide-math/src/tolerance/points_equal_2d
language: rust
---

# points_equal_2d

Check if two 2D points are coincident within linear tolerance.

## Signature

```rust
impl ToleranceContext { pub fn points_equal_2d(&self, p1: [f64; 2], p2: [f64; 2]) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if two 2D points are coincident within linear tolerance.
[must_use]

## Source
Lines 66–70 in `crates/oxide-math/src/tolerance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tolerance](/crates/oxide-math/src/tolerance.md) |
