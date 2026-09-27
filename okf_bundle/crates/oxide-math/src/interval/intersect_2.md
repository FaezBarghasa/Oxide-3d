---
okf_version: "0.2"
type: Function
title: intersect
description: Compute the intersection of two 3D intervals.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/intersect_2
language: rust
---

# intersect

Compute the intersection of two 3D intervals.

## Signature

```rust
impl Interval3d { pub fn intersect(&self, other: &Self) -> Option<Self> }
```

## Visibility

- `pub`

## Docstring

Compute the intersection of two 3D intervals.
[must_use]

## Source
Lines 139–144 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
