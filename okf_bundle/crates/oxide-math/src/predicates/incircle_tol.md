---
okf_version: "0.2"
type: Function
title: incircle_tol
description: Tolerance-aware 2D incircle predicate.
resource: crates/oxide-math/src/predicates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-math"
  - "git:branch:master"
  - "git:repo:Oxide-3d"
timestamp: "2026-09-27T13:57:42Z"
concept_id: crates/oxide-math/src/predicates/incircle_tol
language: rust
---

# incircle_tol

Tolerance-aware 2D incircle predicate.

## Signature

```rust
pub fn incircle_tol(pa: [f64; 2], pb: [f64; 2], pc: [f64; 2], pd: [f64; 2], tol: &ToleranceContext) -> i8
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Tolerance-aware 2D incircle predicate.
[must_use]

## Source
Lines 84–93 in `crates/oxide-math/src/predicates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [predicates](/crates/oxide-math/src/predicates.md) |
| calls | [incircle](/crates/oxide-math/src/predicates/incircle.md) |
