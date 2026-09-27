---
okf_version: "0.2"
type: Function
title: from_center_half_extents
description: Create from center and half-extents.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/from_center_half_extents
language: rust
---

# from_center_half_extents

Create from center and half-extents.

## Signature

```rust
impl Interval3d { pub fn from_center_half_extents(center: [f64; 3], half_extents: [f64; 3]) -> Self }
```

## Visibility

- `pub`

## Docstring

Create from center and half-extents.
[must_use]

## Source
Lines 108–114 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
