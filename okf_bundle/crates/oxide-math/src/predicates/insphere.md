---
okf_version: "0.2"
type: Function
title: insphere
description: Exact adaptive 3D insphere predicate.
resource: crates/oxide-math/src/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:57:42Z"
concept_id: crates/oxide-math/src/predicates/insphere
language: rust
---

# insphere

Exact adaptive 3D insphere predicate.

## Signature

```rust
pub fn insphere(pa: [f64; 3], pb: [f64; 3], pc: [f64; 3], pd: [f64; 3], pe: [f64; 3]) -> f64
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Exact adaptive 3D insphere predicate.
Returns positive if pe is inside sphere through pa, pb, pc, pd; negative if outside; zero if cospherical.
[must_use]

## Source
Lines 98–126 in `crates/oxide-math/src/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-math/src/predicates.md) |
| called_by | [insphere_tol](/crates/oxide-math/src/predicates/insphere_tol.md) |
