---
okf_version: "0.2"
type: Function
title: incircle
description: Exact adaptive 2D incircle predicate.
resource: crates/oxide-math/src/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:57:42Z"
concept_id: crates/oxide-math/src/predicates/incircle
language: rust
---

# incircle

Exact adaptive 2D incircle predicate.

## Signature

```rust
pub fn incircle(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2], pd: [f64; 2]) -> f64
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Exact adaptive 2D incircle predicate.
Returns positive if pd is inside circle through pa, pb, pc; negative if outside; zero if cocircular.
[must_use]

## Source
Lines 73–80 in `crates/oxide-math/src/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-math/src/predicates.md) |
| called_by | [incircle_tol](/crates/oxide-math/src/predicates/incircle_tol.md) |
