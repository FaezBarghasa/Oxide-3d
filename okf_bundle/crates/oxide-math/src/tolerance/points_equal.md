---
okf_version: "0.2"
type: Function
title: points_equal
description: Check if two 3D points are coincident within linear tolerance.
resource: crates/oxide-math/src/tolerance.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:54:05Z"
concept_id: crates/oxide-math/src/tolerance/points_equal
language: rust
---

# points_equal

Check if two 3D points are coincident within linear tolerance.

## Signature

```rust
impl ToleranceContext { pub fn points_equal(&self, p1: [f64; 3], p2: [f64; 3]) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if two 3D points are coincident within linear tolerance.
[must_use]

## Source
Lines 57–62 in `crates/oxide-math/src/tolerance.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tolerance](/crates/oxide-math/src/tolerance.md) |
