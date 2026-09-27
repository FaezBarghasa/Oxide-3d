---
okf_version: "0.2"
type: Function
title: contains
description: Check if a point is contained within the 3D interval.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/contains_2
language: rust
---

# contains

Check if a point is contained within the 3D interval.

## Signature

```rust
impl Interval3d { pub fn contains(&self, point: [f64; 3]) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if a point is contained within the 3D interval.
[must_use]

## Source
Lines 118–120 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
