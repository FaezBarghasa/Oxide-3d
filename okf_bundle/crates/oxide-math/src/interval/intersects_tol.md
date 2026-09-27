---
okf_version: "0.2"
type: Function
title: intersects_tol
description: Check intersection with tolerance margin.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/intersects_tol
language: rust
---

# intersects_tol

Check intersection with tolerance margin.

## Signature

```rust
impl Interval3d { pub fn intersects_tol(&self, other: &Self, tol: f64) -> bool }
```

## Visibility

- `pub`

## Docstring

Check intersection with tolerance margin.
[must_use]

## Source
Lines 130–135 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
