---
okf_version: "0.2"
type: Function
title: new
description: Create a new 3D interval from min and max points.
resource: crates/oxide-math/src/interval.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T14:02:20Z"
concept_id: crates/oxide-math/src/interval/new_2
language: rust
---

# new

Create a new 3D interval from min and max points.

## Signature

```rust
impl Interval3d { pub fn new(min: [f64; 3], max: [f64; 3]) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new 3D interval from min and max points.
[must_use]

## Source
Lines 98–104 in `crates/oxide-math/src/interval.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interval](/crates/oxide-math/src/interval.md) |
